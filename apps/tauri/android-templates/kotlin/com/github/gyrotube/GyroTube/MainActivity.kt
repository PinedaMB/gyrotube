package com.github.gyrotube.GyroTube

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.view.View
import android.webkit.JavascriptInterface
import android.webkit.WebSettings
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : TauriActivity() {
  private var activeWebView: WebView? = null
  @Volatile
  private var playbackActive: Boolean = false
  @Volatile
  private var pendingSpinCode: String? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    captureSpinDeepLink(intent)

    // Request runtime notification permission on Android 13+ so the persistent
    // media playback notification is visible in the shade and on the lock screen.
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
      if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS)
        != PackageManager.PERMISSION_GRANTED
      ) {
        ActivityCompat.requestPermissions(
          this,
          arrayOf(Manifest.permission.POST_NOTIFICATIONS),
          1002
        )
      }
    }

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

    PlaybackForegroundService.onMediaAction = { action ->
      runOnUiThread {
        activeWebView?.onResume()
        activeWebView?.resumeTimers()
        activeWebView?.evaluateJavascript(
          "window.__GYROTUBE_MEDIA_ACTION__ && window.__GYROTUBE_MEDIA_ACTION__('$action');",
          null
        )
      }
    }
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

  override fun onNewIntent(intent: Intent) {
    super.onNewIntent(intent)
    setIntent(intent)
    captureSpinDeepLink(intent)
    dispatchPendingSpinLink()
  }

  private fun captureSpinDeepLink(intent: Intent?) {
    val uri = intent?.data ?: return
    if (uri.scheme.equals("gyrotube", ignoreCase = true) &&
      uri.host.equals("spin", ignoreCase = true)
    ) {
      val code = uri.pathSegments.firstOrNull()?.trim()
      if (!code.isNullOrEmpty()) {
        pendingSpinCode = code
      }
    }
  }

  private fun dispatchPendingSpinLink() {
    val code = pendingSpinCode ?: return
    val wv = activeWebView ?: return
    pendingSpinCode = null
    val escaped = code.replace("\\", "\\\\").replace("'", "\\'")
    runOnUiThread {
      wv.evaluateJavascript(
        "if (window.__GYROTUBE_SPIN_LINK__) { window.__GYROTUBE_SPIN_LINK__('$escaped'); }",
        null
      )
    }
  }

  override fun onDestroy() {
    PlaybackForegroundService.onMediaAction = null
    stopService(Intent(this, PlaybackForegroundService::class.java))
    activeWebView = null
    super.onDestroy()
  }

  private inner class AndroidPlaybackBridge {
    @JavascriptInterface
    fun consumePendingSpinLink(): String {
      val code = pendingSpinCode ?: return ""
      pendingSpinCode = null
      return code
    }

    @JavascriptInterface
    fun setPlaybackActive(
      playing: Boolean,
      title: String,
      artist: String,
      coverUrl: String,
      hasTrack: Boolean,
      positionMs: Long,
      durationMs: Long
    ) {
      playbackActive = playing
      runOnUiThread {
        if (!hasTrack) {
          stopService(Intent(this@MainActivity, PlaybackForegroundService::class.java))
          return@runOnUiThread
        }
        val intent = Intent(this@MainActivity, PlaybackForegroundService::class.java).apply {
          putExtra(PlaybackForegroundService.EXTRA_TITLE, title.ifBlank { "GyroTube" })
          putExtra(PlaybackForegroundService.EXTRA_ARTIST, artist)
          putExtra(PlaybackForegroundService.EXTRA_COVER_URL, coverUrl)
          putExtra(PlaybackForegroundService.EXTRA_PLAYING, playing)
          putExtra(PlaybackForegroundService.EXTRA_POSITION_MS, positionMs.coerceAtLeast(0L))
          putExtra(PlaybackForegroundService.EXTRA_DURATION_MS, durationMs.coerceAtLeast(0L))
        }
        try {
          ContextCompat.startForegroundService(this@MainActivity, intent)
        } catch (_: Throwable) {
          // Ignore OS background start restrictions if already backgrounded
        }
        if (playing) {
          activeWebView?.onResume()
          activeWebView?.resumeTimers()
        }
      }
    }
  }
}
