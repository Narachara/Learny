package com.learny.app

import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : CustomMainActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
  }
}