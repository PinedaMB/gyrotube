package com.github.gyrotube.GyroTube

import android.os.Bundle
import android.view.View
import android.webkit.WebSettings
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)

    // Keep the WebView out of the status bar / navigation bar / keyboard areas so the
    // header (search) and bottom navigation are never covered by system UI.
    val content = findViewById<View>(android.R.id.content)
    ViewCompat.setOnApplyWindowInsetsListener(content) { view, insets ->
      val bars = insets.getInsets(
        WindowInsetsCompat.Type.systemBars() or WindowInsetsCompat.Type.displayCutout()
      )
      val ime = insets.getInsets(WindowInsetsCompat.Type.ime())
      view.setPadding(bars.left, bars.top, bars.right, maxOf(bars.bottom, ime.bottom))
      WindowInsetsCompat.CONSUMED
    }
  }

  override fun onWebViewCreate(webView: WebView) {
    super.onWebViewCreate(webView)
    // Allow async HTML5 <audio>.play() after Tauri IPC resolves the stream URL,
    // and allow loading http://127.0.0.1:<port>/stream/... from https://tauri.localhost.
    webView.settings.mediaPlaybackRequiresUserGesture = false
    webView.settings.mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
  }
}
