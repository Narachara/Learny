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

@TauriPlugin
class ExamplePlugin(private val activity: Activity) : Plugin(activity) {

    private val PICK_IMAGE_REQUEST_CODE = 1
    private var pendingInvoke: Invoke? = null

    // ================================
    // Command: pickImage
    // ================================
    @Command
    fun pickImage(invoke: Invoke) {
        pendingInvoke = invoke

        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
            type = "image/*"
            addCategory(Intent.CATEGORY_OPENABLE)
        }

        activity.startActivityForResult(intent, PICK_IMAGE_REQUEST_CODE)
    }

    // ================================
    // Activity result handler
    // ================================
    fun handleActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        if (requestCode != PICK_IMAGE_REQUEST_CODE) return

        val invoke = pendingInvoke
        pendingInvoke = null

        if (resultCode != Activity.RESULT_OK || data?.data == null) {
            // User cancelled → NOT an error
            val ret = JSObject()
            ret.put("path", null)
            invoke?.resolve(ret)
            return
        }

        val uri = data.data!!
        val result = copyUriToFilesDir(uri)

        if (result == null) {
            invoke?.reject("Failed to import image")
            return
        }

        val (_, virtualPath) = result

        val ret = JSObject()
        ret.put("path", virtualPath)
        invoke?.resolve(ret)
    }

    // ================================
    // Copy image into app Files/
    // ================================
    private fun copyUriToFilesDir(uri: Uri): Pair<File, String>? {
        return try {
            val filesDir = activity.filesDir
            val extension = guessExtension(uri) ?: "png"
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
    // MIME → extension helper
    // ================================
    private fun guessExtension(uri: Uri): String? {
        val mime = activity.contentResolver.getType(uri) ?: return null
        return MimeTypeMap.getSingleton()
            .getExtensionFromMimeType(mime)
    }
}