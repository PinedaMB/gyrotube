package com.github.gyrotube.GyroTube

import android.content.Intent
import android.os.Bundle
import android.view.View
import android.webkit.JavascriptInterface
import android.webkit.WebSettings
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import androidx.core.content.ContextCompat
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : TauriActivity() {
  private var activeWebView: WebView? = null
  @Volatile
  private var playbackActive: Boolean = false

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
    activeWebView = webView
    // Allow async HTML5 <audio>.play() after Tauri IPC resolves the stream URL,
    // and allow loading http://127.0.0.1:<port>/stream/... from https://tauri.localhost.
    webView.settings.mediaPlaybackRequiresUserGesture = false
    webView.settings.mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
    webView.addJavascriptInterface(AndroidPlaybackBridge(), "__GYROTUBE_ANDROID__")
  }

  override fun onPause() {
    super.onPause()
    // WryActivity.onPause() calls mWebView.onPause(), which suspends JS execution,
    // timers, and HTML5 <audio> `ended` events while backgrounded. Keep the WebView
    // active when music is playing so track-to-track transitions continue seamlessly.
    if (playbackActive) {
      activeWebView?.onResume()
      activeWebView?.resumeTimers()
    }
  }

  override fun onDestroy() {
    stopService(Intent(this, PlaybackForegroundService::class.java))
    activeWebView = null
    super.onDestroy()
  }

  private inner class AndroidPlaybackBridge {
    @JavascriptInterface
    fun setPlaybackActive(playing: Boolean, title: String, artist: String) {
      playbackActive = playing
      runOnUiThread {
        val intent = Intent(this@MainActivity, PlaybackForegroundService::class.java).apply {
          putExtra(PlaybackForegroundService.EXTRA_TITLE, title.ifBlank { "GyroTube" })
          putExtra(PlaybackForegroundService.EXTRA_ARTIST, artist.ifBlank { "Playing music" })
        }
        if (playing) {
          try {
            ContextCompat.startForegroundService(this@MainActivity, intent)
          } catch (_: Throwable) {
            // Ignore OS background start restrictions if already backgrounded
          }
          activeWebView?.onResume()
          activeWebView?.resumeTimers()
        } else {
          stopService(Intent(this@MainActivity, PlaybackForegroundService::class.java))
        }
      }
    }
  }
}
