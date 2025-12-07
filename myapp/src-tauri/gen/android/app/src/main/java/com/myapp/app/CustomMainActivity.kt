package com.myapp.app

import android.content.Intent
import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import app.tauri.TauriActivity
import com.plugin.bliet.ExamplePlugin  // Import your plugin

class CustomMainActivity : TauriActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        // Forward the result to your plugin
        val plugin = this.plugins.get("bliet") as? ExamplePlugin
        plugin?.handleActivityResult(requestCode, resultCode, data)
    }
}