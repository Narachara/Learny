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

    private val PICK_IMAGE_REQUEST_CODE = 1
    private val PICK_ARCHIVE_REQUEST_CODE = 2
    private val PICK_IMPORT_REQUEST_CODE = 3   // ✅ unique
    private val SAVE_EXPORT_REQUEST_CODE = 4

    private var pendingInvoke: Invoke? = null
    private var pendingExportData: ByteArray? = null
    private var pendingExportInvoke: Invoke? = null
   

    // ================================
    // pickImage
    // ================================
    @Command
    fun pickImage(invoke: Invoke) {
        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
            type = "image/*"
            addCategory(Intent.CATEGORY_OPENABLE)
        }

        startActivityForResult(intent) { result ->
            if (result.resultCode != Activity.RESULT_OK || result.data?.data == null) {
                invoke.resolve(null)
                return@startActivityForResult
            }

            val uri = result.data!!.data!!
            val resultFile = copyUriToFilesDir(uri)
                ?: run {
                    invoke.reject("Failed to import image")
                    return@startActivityForResult
                }

            val (_, virtualPath) = resultFile
            val ret = JSObject()
            ret.put("path", virtualPath)
            invoke.resolve(ret)
        }
    }


    // ================================
    // pickArchive
    // ================================
    @Command
    fun pickArchive(invoke: Invoke) {
        pendingInvoke = invoke

        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
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
        }

        activity.startActivityForResult(intent, PICK_ARCHIVE_REQUEST_CODE)
    }

    // ================================
    // pickImportFile (BYTES)
    // ================================
    @Command
    fun pickImportFile(invoke: Invoke) {
        pendingInvoke = invoke

        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
            type = "*/*"
            addCategory(Intent.CATEGORY_OPENABLE)
        }

        activity.startActivityForResult(intent, PICK_IMPORT_REQUEST_CODE)
    }

    @Command
    fun saveExportBytes(invoke: Invoke) {
        val args = invoke.getArgs()
        val data = args.getString("data") ?: run {
            invoke.reject("Missing export data")
            return
        }

        val fileName = args.getString("fileName") ?: "export.zip"
        val bytes = Base64.decode(data, Base64.DEFAULT)

        val intent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "application/zip"
            putExtra(Intent.EXTRA_TITLE, fileName)
        }

        startActivityForResult(intent) { result ->
            if (result.resultCode != Activity.RESULT_OK || result.data?.data == null) {
                invoke.resolve(null)
                return@startActivityForResult
            }

            try {
                val uri = result.data!!.data!!
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
        }
    }



    // ================================
    // Activity result handler
    // ================================
    fun handleActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {

    if (requestCode == SAVE_EXPORT_REQUEST_CODE) {
        val invoke = pendingExportInvoke
        val bytes = pendingExportData

        pendingExportInvoke = null
        pendingExportData = null

        if (invoke == null) return

        if (resultCode != Activity.RESULT_OK || data?.data == null || bytes == null) {
            invoke.resolve(null)
            return
        }

        try {
            val uri = data.data!!

            val pfd = activity.contentResolver.openFileDescriptor(uri, "wt")
                ?: throw IOException("Failed to open file descriptor")

            FileOutputStream(pfd.fileDescriptor).use { out ->
                out.write(bytes)
                out.flush()
            }

            pfd.close()

            android.util.Log.d(
                "BlietExport",
                "Wrote ${bytes.size} bytes successfully to $uri"
            )

            invoke.resolve(null)
        } catch (e: Exception) {
            android.util.Log.e("BlietExport", "Failed to save export", e)
            invoke.reject("Failed to save export file")
        }

        return
    }


    // ---------- everything else uses pendingInvoke ----------
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
        PICK_IMPORT_REQUEST_CODE -> {
            try {
                val fileBytes = activity.contentResolver.openInputStream(uri)?.readBytes()
                val encoded = Base64.encodeToString(fileBytes, Base64.NO_WRAP)
                val ret = JSObject()
                ret.put("data", encoded)
                invoke.resolve(ret)
            } catch (e: Exception) {
                invoke.reject("Failed to read import file")
            }
        }

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
