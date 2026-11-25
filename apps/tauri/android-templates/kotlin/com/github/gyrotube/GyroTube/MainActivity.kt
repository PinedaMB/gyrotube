package com.github.gyrotube.GyroTube

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.content.res.Configuration
import android.graphics.Color
import android.media.AudioManager
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import android.view.View
import android.webkit.JavascriptInterface
import android.webkit.WebSettings
import android.webkit.WebView
import androidx.activity.OnBackPressedCallback
import androidx.activity.enableEdgeToEdge
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.core.content.FileProvider
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import java.io.File
import java.io.FileOutputStream
import java.net.HttpURLConnection
import java.net.URL
import java.util.concurrent.atomic.AtomicBoolean

class MainActivity : TauriActivity() {
  // Handle Back presses ourselves so exhausting the WebView history moves the app
  // to the background instead of destroying the Activity and killing playback.
  override val handleBackNavigation: Boolean = false

  private var activeWebView: WebView? = null
  @Volatile
  private var playbackActive: Boolean = false
  @Volatile
  private var pendingSpinCode: String? = null
  @Volatile
  private var pendingInstallApk: File? = null
  private val apkDownloading = AtomicBoolean(false)

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    // Route hardware volume keys to media volume even before audio starts playing.
    volumeControlStream = AudioManager.STREAM_MUSIC
    captureSpinDeepLink(intent)

    onBackPressedDispatcher.addCallback(this, object : OnBackPressedCallback(true) {
      override fun handleOnBackPressed() {
        val wv = activeWebView
        if (wv != null && wv.canGoBack()) {
          wv.goBack()
          return
        }
        if (wv != null) {
          wv.evaluateJavascript(
            "Boolean(window.__GYROTUBE_DISMISS_OVERLAY__ && window.__GYROTUBE_DISMISS_OVERLAY__())"
          ) { dismissed ->
            if (dismissed != "true") {
              moveTaskToBack(true)
            }
          }
        } else {
          moveTaskToBack(true)
        }
      }
    })

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
    val isNight = (resources.configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK) == Configuration.UI_MODE_NIGHT_YES
    applySystemBars(if (isNight) "#1e1c26" else "#ffffff", isNight)
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
    webView.settings.setSupportZoom(false)
    webView.settings.builtInZoomControls = false
    webView.settings.displayZoomControls = false
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

  override fun onResume() {
    super.onResume()
    val pendingApk = pendingInstallApk
    if (pendingApk != null && pendingApk.exists()) {
      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O || packageManager.canRequestPackageInstalls()) {
        pendingInstallApk = null
        launchApkInstaller(pendingApk)
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

  @Suppress("DEPRECATION")
  private fun applySystemBars(bgHex: String, isDark: Boolean) {
    val color = try {
      Color.parseColor(bgHex)
    } catch (_: Throwable) {
      if (isDark) Color.parseColor("#1e1c26") else Color.WHITE
    }
    window.decorView.setBackgroundColor(color)
    findViewById<View>(android.R.id.content)?.setBackgroundColor(color)
    if (Build.VERSION.SDK_INT < 35) {
      window.statusBarColor = color
      window.navigationBarColor = color
    }
    val controller = WindowCompat.getInsetsController(window, window.decorView)
    controller.isAppearanceLightStatusBars = !isDark
    controller.isAppearanceLightNavigationBars = !isDark
  }

  private fun notifyUpdateProgress(status: String, pct: Int, error: String = "") {
    val safeErr = error
      .replace("\\", "\\\\")
      .replace("'", "\\'")
      .replace("\n", " ")
      .replace("\r", " ")
    runOnUiThread {
      activeWebView?.evaluateJavascript(
        "if (window.__GYROTUBE_ANDROID_UPDATE__) { window.__GYROTUBE_ANDROID_UPDATE__('$status', $pct, '$safeErr'); }",
        null
      )
    }
  }

  private fun launchApkInstaller(apkFile: File) {
    try {
      val uri = FileProvider.getUriForFile(this, "${packageName}.fileprovider", apkFile)
      val intent = Intent(Intent.ACTION_VIEW).apply {
        setDataAndType(uri, "application/vnd.android.package-archive")
        addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_ACTIVITY_NEW_TASK)
      }
      startActivity(intent)
      notifyUpdateProgress("ready", 100)
    } catch (t: Throwable) {
      notifyUpdateProgress("error", 0, t.message ?: "Failed to launch installer")
    }
  }

  private inner class AndroidPlaybackBridge {
    @JavascriptInterface
    fun setSystemBarsTheme(bgHex: String, isDark: Boolean) {
      runOnUiThread {
        applySystemBars(bgHex, isDark)
      }
    }

    @JavascriptInterface
    fun openExternalUrl(url: String) {
      val trimmed = url.trim()
      if (!trimmed.startsWith("https://") && !trimmed.startsWith("http://")) return
      runOnUiThread {
        try {
          val intent = Intent(Intent.ACTION_VIEW, Uri.parse(trimmed)).apply {
            addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
          }
          startActivity(intent)
        } catch (_: Throwable) {
          // Ignore if no browser is available
        }
      }
    }

    @JavascriptInterface
    fun downloadAndInstallApk(url: String) {
      val trimmed = url.trim()
      if (!trimmed.startsWith("https://")) {
        notifyUpdateProgress("error", 0, "Invalid update URL")
        return
      }
      if (!apkDownloading.compareAndSet(false, true)) return

      Thread {
        try {
          notifyUpdateProgress("progress", 0)
          var currentUrl = trimmed
          var conn: HttpURLConnection? = null
          for (redirect in 0 until 6) {
            val c = (URL(currentUrl).openConnection() as HttpURLConnection).apply {
              instanceFollowRedirects = false
              connectTimeout = 15000
              readTimeout = 30000
              setRequestProperty("User-Agent", "GyroTube-Android")
            }
            val code = c.responseCode
            if (code in 300..399) {
              val location = c.getHeaderField("Location")
              c.disconnect()
              if (location.isNullOrBlank()) throw IllegalStateException("Empty redirect")
              currentUrl = URL(URL(currentUrl), location).toString()
              continue
            }
            conn = c
            break
          }

          val activeConn = conn ?: throw IllegalStateException("Too many redirects")
          try {
            if (activeConn.responseCode !in 200..299) {
              throw IllegalStateException("HTTP ${activeConn.responseCode}")
            }
            val total = activeConn.contentLengthLong
            val tmpFile = File(cacheDir, "gyrotube-update.apk.part")
            val outFile = File(cacheDir, "gyrotube-update.apk")
            if (tmpFile.exists()) tmpFile.delete()

            activeConn.inputStream.use { input ->
              FileOutputStream(tmpFile).use { output ->
                val buf = ByteArray(16 * 1024)
                var downloaded = 0L
                var lastPct = -2
                var lastNotifyMs = 0L
                while (true) {
                  val read = input.read(buf)
                  if (read <= 0) break
                  output.write(buf, 0, read)
                  downloaded += read
                  val now = System.currentTimeMillis()
                  val pct = if (total > 0L) ((downloaded * 100L) / total).toInt().coerceIn(0, 100) else -1
                  if (pct != lastPct && (now - lastNotifyMs >= 180L || pct == 100)) {
                    lastPct = pct
                    lastNotifyMs = now
                    notifyUpdateProgress("progress", pct)
                  }
                }
              }
            }

            if (outFile.exists()) outFile.delete()
            if (!tmpFile.renameTo(outFile)) {
              tmpFile.copyTo(outFile, overwrite = true)
              tmpFile.delete()
            }

            runOnUiThread {
              if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O && !packageManager.canRequestPackageInstalls()) {
                pendingInstallApk = outFile
                notifyUpdateProgress("ready", 100)
                try {
                  val settingsIntent = Intent(
                    Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES,
                    Uri.parse("package:$packageName")
                  ).apply {
                    addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                  }
                  startActivity(settingsIntent)
                } catch (_: Throwable) {
                  launchApkInstaller(outFile)
                }
              } else {
                launchApkInstaller(outFile)
              }
            }
          } finally {
            activeConn.disconnect()
          }
        } catch (t: Throwable) {
          notifyUpdateProgress("error", 0, t.message ?: "Download failed")
        } finally {
          apkDownloading.set(false)
        }
      }.start()
    }

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
