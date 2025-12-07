package com.plugin.bliet

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.provider.OpenableColumns
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Channel

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
                    // Get the file path from the URI
                    val filePath = getPathFromUri(uri)
                    // Resolve the pending invoke with the file path
                    val ret = JSObject()
                    ret.put("path", filePath)
                    pendingInvoke?.resolve(ret)
                    pendingInvoke = null
                }
            } else {
                // User canceled or error occurred
                pendingInvoke?.reject("User canceled or error occurred")
                pendingInvoke = null
            }
        }
    }

    private fun getPathFromUri(uri: Uri): String {
        // Simplified: Return the URI string or extract the path
        // For a real app, use DocumentFile or similar
        var result: String? = null
        val cursor = activity.contentResolver.query(uri, null, null, null, null)
        cursor?.use {
            it.moveToFirst()
            val nameIndex = it.getColumnIndex(OpenableColumns.DISPLAY_NAME)
            result = it.getString(nameIndex)
        }
        return result ?: uri.toString()
    }
}