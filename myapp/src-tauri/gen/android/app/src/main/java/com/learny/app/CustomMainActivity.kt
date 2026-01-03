package com.learny.app

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import androidx.core.app.ActivityCompat
import com.plugin.bliet.ExamplePlugin

open class CustomMainActivity : TauriActivity() {

    companion object {
        private const val REQUEST_CODE_READ_STORAGE = 101
        private const val REQUEST_CODE_WRITE_STORAGE = 102
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)

        // Request runtime permissions for file access
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            if (checkSelfPermission(Manifest.permission.READ_EXTERNAL_STORAGE) != PackageManager.PERMISSION_GRANTED) {
                ActivityCompat.requestPermissions(
                    this,
                    arrayOf(Manifest.permission.READ_EXTERNAL_STORAGE),
                    REQUEST_CODE_READ_STORAGE
                )
            }
            if (checkSelfPermission(Manifest.permission.WRITE_EXTERNAL_STORAGE) != PackageManager.PERMISSION_GRANTED) {
                ActivityCompat.requestPermissions(
                    this,
                    arrayOf(Manifest.permission.WRITE_EXTERNAL_STORAGE),
                    REQUEST_CODE_WRITE_STORAGE
                )
            }
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        // Handle permission results if needed
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)

        try {
            // Access the plugins field using reflection
            val pluginsField = pluginManager.javaClass.getDeclaredField("plugins")
            pluginsField.isAccessible = true
            val plugins = pluginsField.get(pluginManager) as? Map<*, *>

            plugins?.forEach { (_, handle) ->
                try {
                    // Ensure handle is not null
                    if (handle != null) {
                        // Access the instance field
                        val instanceField = handle.javaClass.getDeclaredField("instance")
                        instanceField.isAccessible = true
                        val instance = instanceField.get(handle)

                        // Check if the instance is of type ExamplePlugin
                        if (instance is ExamplePlugin) {
                            instance.handleActivityResult(requestCode, resultCode, data)
                        }
                    }
                } catch (e: Exception) {
                    e.printStackTrace()
                }
            }
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}