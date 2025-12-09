package com.plugin.bliet

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.os.Environment
import android.provider.OpenableColumns
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Channel
import java.io.File
import java.io.FileOutputStream
import java.io.IOException

@TauriPlugin
class ExamplePlugin(private val activity: Activity) : Plugin(activity) {
    private val PICK_FILE_REQUEST_CODE = 1
    private var pendingInvoke: Invoke? = null

    @Command
    fun pickImage(invoke: Invoke) {
        // Store the invoke object for later
        pendingInvoke = invoke

        // Create an intent to open a file picker
        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
            type = "*/*" // Allow all file types
            addCategory(Intent.CATEGORY_OPENABLE)
        }

        // Start the activity for result
        activity.startActivityForResult(intent, PICK_FILE_REQUEST_CODE)
    }

    // Call this from your Activity's onActivityResult
    fun handleActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        if (requestCode == PICK_FILE_REQUEST_CODE) {
            if (resultCode == Activity.RESULT_OK) {
                data?.data?.let { uri ->
                    val savedFile = copyUriToAppStorage(uri)
                    if (savedFile != null) {
                        val ret = JSObject()
                        ret.put("path", savedFile.absolutePath)
                        ret.put("name", savedFile.name)
                        ret.put("uri", uri.toString())
                        pendingInvoke?.resolve(ret)
                    } else {
                        pendingInvoke?.reject("Unable to copy selected file")
                    }
                    pendingInvoke = null
                } ?: run {
                    pendingInvoke?.reject("No data returned from picker")
                    pendingInvoke = null
                }
            } else {
                // User canceled or error occurred
                pendingInvoke?.reject("User canceled or error occurred")
                pendingInvoke = null
            }
        }
    }

    private fun copyUriToAppStorage(uri: Uri): File? {
        return try {
            val displayName = getDisplayName(uri) ?: "picked_${System.currentTimeMillis()}"
            val mime = activity.contentResolver.getType(uri).orEmpty()
            val destDir = getDestinationDir(mime)
            if (!destDir.exists()) {
                destDir.mkdirs()
            }
            val targetFile = uniqueFile(destDir, displayName)
            activity.contentResolver.openInputStream(uri)?.use { input ->
                FileOutputStream(targetFile).use { output ->
                    input.copyTo(output)
                }
            } ?: return null
            targetFile
        } catch (e: IOException) {
            null
        }
    }

    private fun getDestinationDir(mime: String): File {
        val dir = when {
            mime.startsWith("image/") -> Environment.DIRECTORY_PICTURES
            mime.startsWith("video/") -> Environment.DIRECTORY_MOVIES
            mime.startsWith("audio/") -> Environment.DIRECTORY_MUSIC
            else -> Environment.DIRECTORY_DOCUMENTS
        }
        return activity.getExternalFilesDir(dir) ?: activity.filesDir
    }

    private fun uniqueFile(dir: File, name: String): File {
        var file = File(dir, name)
        if (!file.exists()) {
            return file
        }
        val dotIndex = name.lastIndexOf('.')
        val base = if (dotIndex > 0) name.substring(0, dotIndex) else name
        val ext = if (dotIndex > 0) name.substring(dotIndex) else ""
        var index = 1
        while (file.exists()) {
            val newName = "${base}_$index$ext"
            file = File(dir, newName)
            index++
        }
        return file
    }

    private fun getDisplayName(uri: Uri): String? {
        var result: String? = null
        val cursor = activity.contentResolver.query(uri, null, null, null, null)
        cursor?.use {
            if (it.moveToFirst()) {
                val nameIndex = it.getColumnIndex(OpenableColumns.DISPLAY_NAME)
                if (nameIndex != -1) {
                    result = it.getString(nameIndex)
                }
            }
        }
        return result
    }
}
