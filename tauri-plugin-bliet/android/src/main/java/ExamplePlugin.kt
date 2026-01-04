package com.plugin.bliet

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.provider.OpenableColumns
import android.webkit.MimeTypeMap
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import app.tauri.plugin.JSObject
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.util.UUID
import android.util.Base64


@TauriPlugin
class ExamplePlugin(private val activity: Activity) : Plugin(activity) {
    private var pendingExportData: ByteArray? = null

    private val PICK_IMAGE_REQUEST_CODE = 1
    private val PICK_ARCHIVE_REQUEST_CODE = 2
    private val PICK_IMPORT_REQUEST_CODE = 3   // ✅ unique
    private val SAVE_EXPORT_REQUEST_CODE = 4

    private var pendingInvoke: Invoke? = null

    // ================================
    // pickImage
    // ================================
    @Command
    fun pickImage(invoke: Invoke) {
        pendingInvoke = invoke

    val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
        addCategory(Intent.CATEGORY_OPENABLE)
        type = "image/*"
        flags = Intent.FLAG_GRANT_READ_URI_PERMISSION or
                Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION
    }

        activity.startActivityForResult(intent, PICK_IMAGE_REQUEST_CODE)
    }

    // ================================
    // pickArchive
    // ================================
    @Command
    fun pickArchive(invoke: Invoke) {
        pendingInvoke = invoke

        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
            type = "*/*"
            addCategory(Intent.CATEGORY_OPENABLE)
            putExtra(
                Intent.EXTRA_MIME_TYPES,
                arrayOf(
                    "application/zip",
                    "application/x-tar",
                    "application/gzip",
                    "application/x-7z-compressed",
                    "application/x-rar-compressed"
                )
            )
            flags = Intent.FLAG_GRANT_READ_URI_PERMISSION or
                Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION
        }

        activity.startActivityForResult(intent, PICK_ARCHIVE_REQUEST_CODE)
    }

    // ================================
    // pickImportFile (BYTES)
    // ================================
    @Command
    fun pickImportFile(invoke: Invoke) {
        pendingInvoke = invoke

        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
            type = "*/*"
            addCategory(Intent.CATEGORY_OPENABLE)
            flags = Intent.FLAG_GRANT_READ_URI_PERMISSION or
                    Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION
        }

        activity.startActivityForResult(intent, PICK_IMPORT_REQUEST_CODE)
    }

    // ================================
    // exportBytes
    // ================================
    @Command
    fun saveExportBytes(invoke: Invoke) {
        val args = invoke.getArgs()

        val data = args.getString("data") ?: run {
            invoke.reject("Missing export data")
            return
        }

        val fileName = args.getString("fileName") ?: "export.zip"
        val bytes = Base64.decode(data, Base64.NO_WRAP)

        val intent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "application/zip"
            putExtra(Intent.EXTRA_TITLE, fileName)
            flags = Intent.FLAG_GRANT_WRITE_URI_PERMISSION or
                    Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION
        }





        pendingInvoke = invoke
        pendingExportData = bytes

        activity.startActivityForResult(intent, SAVE_EXPORT_REQUEST_CODE)
    }


    // ================================
    // Activity result handler
    // ================================
fun handleActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {

    // ---------- EXPORT ----------
    if (requestCode == SAVE_EXPORT_REQUEST_CODE) {
        val invoke = pendingInvoke
        val bytes = pendingExportData

        pendingInvoke = null
        pendingExportData = null

        if (invoke == null) return

        if (resultCode != Activity.RESULT_OK || data?.data == null || bytes == null) {
            invoke.resolve(null)
            return
        }

        try {
            val uri = data.data!!
            
            activity.contentResolver.takePersistableUriPermission(
                uri,
                Intent.FLAG_GRANT_WRITE_URI_PERMISSION
            )
            
            val pfd = activity.contentResolver.openFileDescriptor(uri, "wt")
                ?: throw IOException("Failed to open file descriptor")

            FileOutputStream(pfd.fileDescriptor).use { out ->
                out.write(bytes)
                out.flush()
            }
            pfd.close()

            invoke.resolve(null)
        } catch (e: Exception) {
            invoke.reject("Failed to save export file")
        }
        return
    }

    // ---------- IMAGE / ARCHIVE / IMPORT ----------
    val invoke = pendingInvoke ?: return
    pendingInvoke = null

    if (resultCode != Activity.RESULT_OK || data?.data == null) {
        val ret = JSObject()
        when (requestCode) {
            PICK_IMPORT_REQUEST_CODE -> ret.put("data", null)
            else -> ret.put("path", null)
        }
        invoke.resolve(ret)
        return
    }

    val uri = data.data!!

    when (requestCode) {

        // ---------- IMPORT (bytes) ----------
        PICK_IMPORT_REQUEST_CODE -> {
            try {
                activity.contentResolver.takePersistableUriPermission(
                uri,
                Intent.FLAG_GRANT_READ_URI_PERMISSION
            )
                val bytes = activity.contentResolver
                    .openInputStream(uri)
                    ?.readBytes()

                val encoded = Base64.encodeToString(bytes, Base64.NO_WRAP)
                val ret = JSObject()
                ret.put("data", encoded)
                invoke.resolve(ret)
            } catch (e: Exception) {
                invoke.reject("Failed to read import file")
            }
        }

        // ---------- IMAGE / ARCHIVE (path) ----------
        PICK_IMAGE_REQUEST_CODE,
        PICK_ARCHIVE_REQUEST_CODE -> {
            val result = copyUriToFilesDir(uri)
            if (result == null) {
                invoke.reject("Failed to import file")
                return
            }

            val (_, virtualPath) = result
            val ret = JSObject()
            ret.put("path", virtualPath)
            invoke.resolve(ret)
        }
    }
}


    // ================================
    // Copy file into app Files/
    // ================================
    private fun copyUriToFilesDir(uri: Uri): Pair<File, String>? {
        return try {
            val filesDir = activity.filesDir
            val extension = guessExtension(uri) ?: "bin"
            val fileName = "${UUID.randomUUID()}.$extension"

            val targetFile = File(filesDir, fileName)

            activity.contentResolver.openInputStream(uri)?.use { input ->
                FileOutputStream(targetFile).use { output ->
                    input.copyTo(output)
                }
            } ?: return null

            val virtualPath = "files/$fileName"
            targetFile to virtualPath
        } catch (e: IOException) {
            null
        }
    }

    // ================================
    // MIME / filename → extension
    // ================================
    private fun guessExtension(uri: Uri): String? {
        val resolver = activity.contentResolver

        resolver.query(uri, null, null, null, null)?.use { cursor ->
            val nameIndex = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
            if (nameIndex != -1 && cursor.moveToFirst()) {
                val name = cursor.getString(nameIndex)
                val dot = name.lastIndexOf('.')
                if (dot != -1) {
                    return name.substring(dot + 1)
                }
            }
        }

        val mime = resolver.getType(uri) ?: return null
        return MimeTypeMap.getSingleton().getExtensionFromMimeType(mime)
    }
}