package com.myapp.app

import android.content.Intent
import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import app.tauri.plugin.PluginHandle
import app.tauri.plugin.PluginManager
import com.plugin.bliet.ExamplePlugin

open class CustomMainActivity : TauriActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        findPlugin<ExamplePlugin>("bliet")?.handleActivityResult(requestCode, resultCode, data)
    }

    @Suppress("UNCHECKED_CAST")
    private fun <T> findPlugin(id: String): T? {
        return try {
            val field = PluginManager::class.java.getDeclaredField("plugins").apply {
                isAccessible = true
            }
            val pluginMap = field.get(pluginManager) as? Map<*, *>
            val handle = pluginMap?.get(id) as? PluginHandle
            handle?.instance as? T
        } catch (e: Exception) {
            null
        }
    }
}
