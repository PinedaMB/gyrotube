package com.github.gyrotube.GyroTube

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.ServiceInfo
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.graphics.drawable.Icon
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.media.MediaMetadata
import android.media.session.MediaSession
import android.media.session.PlaybackState
import android.os.Build
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.os.PowerManager
import java.io.ByteArrayOutputStream
import java.net.HttpURLConnection
import java.net.URL
import java.util.concurrent.Executors

/**
 * Foreground media playback service keeping GyroTube's local AudioStreamer,
 * WebView audio, lock-screen MediaSession, and persistent media notification active.
 */
class PlaybackForegroundService : Service() {
    private var wakeLock: PowerManager.WakeLock? = null
    private var mediaSession: MediaSession? = null
    private var audioManager: AudioManager? = null
    private var audioFocusRequest: AudioFocusRequest? = null
    private var hasAudioFocus = false
    private var pausedByTransientFocus = false
    private var isDucked = false
    private val mainHandler = Handler(Looper.getMainLooper())
    private val artExecutor = Executors.newSingleThreadExecutor()

    private var currentTitle = "Gyro"
    private var currentArtist = ""
    private var currentPlaying = false
    private var currentPositionMs = 0L
    private var currentDurationMs = 0L
    private var currentCoverUrl: String? = null
    private var currentCoverBitmap: Bitmap? = null
    private var noisyReceiverRegistered = false

    private val audioFocusChangeListener = AudioManager.OnAudioFocusChangeListener { focusChange ->
        when (focusChange) {
            AudioManager.AUDIOFOCUS_LOSS -> {
                hasAudioFocus = false
                pausedByTransientFocus = false
                if (isDucked) {
                    isDucked = false
                    onMediaAction?.invoke("unduck")
                }
                if (currentPlaying) {
                    onMediaAction?.invoke("pause")
                }
            }
            AudioManager.AUDIOFOCUS_LOSS_TRANSIENT -> {
                hasAudioFocus = false
                if (currentPlaying) {
                    pausedByTransientFocus = true
                    onMediaAction?.invoke("pause")
                }
            }
            AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK -> {
                if (currentPlaying && !isDucked) {
                    isDucked = true
                    onMediaAction?.invoke("duck")
                }
            }
            AudioManager.AUDIOFOCUS_GAIN -> {
                hasAudioFocus = true
                if (isDucked) {
                    isDucked = false
                    onMediaAction?.invoke("unduck")
                }
                if (pausedByTransientFocus) {
                    pausedByTransientFocus = false
                    if (!currentPlaying) {
                        onMediaAction?.invoke("play")
                    }
                }
            }
        }
    }

    private val becomingNoisyReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            if (intent?.action == AudioManager.ACTION_AUDIO_BECOMING_NOISY && currentPlaying) {
                onMediaAction?.invoke("pause")
            }
        }
    }

    companion object {
        const val CHANNEL_ID = "gyrotube_playback"
        const val NOTIFICATION_ID = 1001
        const val EXTRA_TITLE = "title"
        const val EXTRA_ARTIST = "artist"
        const val EXTRA_COVER_URL = "cover_url"
        const val EXTRA_PLAYING = "playing"
        const val EXTRA_POSITION_MS = "position_ms"
        const val EXTRA_DURATION_MS = "duration_ms"

        const val ACTION_PREV = "com.github.gyrotube.GyroTube.ACTION_PREV"
        const val ACTION_TOGGLE = "com.github.gyrotube.GyroTube.ACTION_TOGGLE"
        const val ACTION_NEXT = "com.github.gyrotube.GyroTube.ACTION_NEXT"

        @Volatile
        var onMediaAction: ((String) -> Unit)? = null
    }

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
        val pm = getSystemService(PowerManager::class.java)
        wakeLock = pm?.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "Gyro::PlaybackWakeLock")?.apply {
            setReferenceCounted(false)
        }
        audioManager = getSystemService(AudioManager::class.java)
        mediaSession = MediaSession(this, "GyroSession").apply {
            setCallback(object : MediaSession.Callback() {
                override fun onPlay() {
                    pausedByTransientFocus = false
                    onMediaAction?.invoke("play")
                }

                override fun onPause() {
                    pausedByTransientFocus = false
                    onMediaAction?.invoke("pause")
                }

                override fun onStop() {
                    pausedByTransientFocus = false
                    abandonAudioFocus()
                    onMediaAction?.invoke("pause")
                    stopForeground(STOP_FOREGROUND_REMOVE)
                    stopSelf()
                }

                override fun onSkipToNext() {
                    onMediaAction?.invoke("next")
                }

                override fun onSkipToPrevious() {
                    onMediaAction?.invoke("prev")
                }

                override fun onSeekTo(pos: Long) {
                    currentPositionMs = pos.coerceAtLeast(0L)
                    updatePlaybackState()
                    onMediaAction?.invoke("seek:$currentPositionMs")
                }
            })
            isActive = true
        }

        val noisyFilter = IntentFilter(AudioManager.ACTION_AUDIO_BECOMING_NOISY)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            registerReceiver(becomingNoisyReceiver, noisyFilter, Context.RECEIVER_NOT_EXPORTED)
        } else {
            @Suppress("DEPRECATION")
            registerReceiver(becomingNoisyReceiver, noisyFilter)
        }
        noisyReceiverRegistered = true
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_PREV -> {
                onMediaAction?.invoke("prev")
                return START_STICKY
            }
            ACTION_TOGGLE -> {
                onMediaAction?.invoke("toggle")
                return START_STICKY
            }
            ACTION_NEXT -> {
                onMediaAction?.invoke("next")
                return START_STICKY
            }
        }

        if (intent != null) {
            currentTitle = intent.getStringExtra(EXTRA_TITLE)?.takeIf { it.isNotBlank() } ?: "Gyro"
            currentArtist = intent.getStringExtra(EXTRA_ARTIST) ?: ""
            currentPlaying = intent.getBooleanExtra(EXTRA_PLAYING, true)
            if (intent.hasExtra(EXTRA_POSITION_MS)) {
                currentPositionMs = intent.getLongExtra(EXTRA_POSITION_MS, 0L).coerceAtLeast(0L)
            }
            if (intent.hasExtra(EXTRA_DURATION_MS)) {
                currentDurationMs = intent.getLongExtra(EXTRA_DURATION_MS, 0L).coerceAtLeast(0L)
            }
            val coverUrl = intent.getStringExtra(EXTRA_COVER_URL)?.takeIf { it.isNotBlank() }
            if (coverUrl != currentCoverUrl) {
                currentCoverUrl = coverUrl
                currentCoverBitmap = null
                if (coverUrl != null) {
                    fetchCoverArtAsync(coverUrl)
                }
            }
        }

        updateSessionAndNotification(startInForeground = currentPlaying)

        if (currentPlaying) {
            requestAudioFocus()
            if (wakeLock?.isHeld == false) {
                wakeLock?.acquire(4 * 60 * 60 * 1000L)
            }
        } else {
            if (!pausedByTransientFocus) {
                abandonAudioFocus()
            }
            if (wakeLock?.isHeld == true) {
                wakeLock?.release()
            }
            // Detach foreground lock while paused so the user can dismiss the notification
            stopForeground(STOP_FOREGROUND_DETACH)
        }

        return START_STICKY
    }

    override fun onTaskRemoved(rootIntent: Intent?) {
        if (!currentPlaying) {
            abandonAudioFocus()
            stopForeground(STOP_FOREGROUND_REMOVE)
            stopSelf()
        }
        super.onTaskRemoved(rootIntent)
    }

    private fun requestAudioFocus() {
        if (hasAudioFocus) return
        val am = audioManager ?: return
        val attrs = AudioAttributes.Builder()
            .setUsage(AudioAttributes.USAGE_MEDIA)
            .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC)
            .build()
        val req = AudioFocusRequest.Builder(AudioManager.AUDIOFOCUS_GAIN)
            .setAudioAttributes(attrs)
            .setAcceptsDelayedFocusGain(true)
            .setWillPauseWhenDucked(false)
            .setOnAudioFocusChangeListener(audioFocusChangeListener, mainHandler)
            .build()
        audioFocusRequest = req
        hasAudioFocus = am.requestAudioFocus(req) == AudioManager.AUDIOFOCUS_REQUEST_GRANTED
    }

    private fun abandonAudioFocus() {
        val am = audioManager ?: return
        audioFocusRequest?.let { am.abandonAudioFocusRequest(it) }
        audioFocusRequest = null
        hasAudioFocus = false
    }

    private fun isAllowedCoverUrl(rawUrl: String): Boolean {
        return try {
            val parsed = URL(rawUrl)
            if (parsed.protocol.equals("https", ignoreCase = true)) return true
            parsed.protocol.equals("http", ignoreCase = true) &&
                (parsed.host == "127.0.0.1" || parsed.host.equals("localhost", ignoreCase = true))
        } catch (_: Throwable) {
            false
        }
    }

    private fun decodeDownsampledBitmap(bytes: ByteArray, maxDim: Int): Bitmap? {
        val bounds = BitmapFactory.Options().apply { inJustDecodeBounds = true }
        BitmapFactory.decodeByteArray(bytes, 0, bytes.size, bounds)
        val w = bounds.outWidth
        val h = bounds.outHeight
        if (w <= 0 || h <= 0) return null
        var sampleSize = 1
        while ((w / sampleSize) > maxDim * 2 || (h / sampleSize) > maxDim * 2) {
            sampleSize *= 2
        }
        val opts = BitmapFactory.Options().apply {
            inSampleSize = sampleSize
            inPreferredConfig = Bitmap.Config.RGB_565
        }
        return BitmapFactory.decodeByteArray(bytes, 0, bytes.size, opts)
    }

    private fun fetchCoverArtAsync(url: String) {
        if (!isAllowedCoverUrl(url)) return
        artExecutor.execute {
            var conn: HttpURLConnection? = null
            try {
                conn = (URL(url).openConnection() as HttpURLConnection).apply {
                    connectTimeout = 4000
                    readTimeout = 4000
                    setRequestProperty("User-Agent", "Gyro/1.0 (Android)")
                }
                if (conn.responseCode !in 200..299) return@execute
                val maxCoverBytes = 5 * 1024 * 1024
                if (conn.contentLength > maxCoverBytes) return@execute
                val baos = ByteArrayOutputStream()
                conn.inputStream.use { input ->
                    val buf = ByteArray(8192)
                    var total = 0
                    while (true) {
                        val read = input.read(buf)
                        if (read <= 0) break
                        total += read
                        if (total > maxCoverBytes) return@execute
                        baos.write(buf, 0, read)
                    }
                }
                val bmp = decodeDownsampledBitmap(baos.toByteArray(), 512)
                if (bmp != null) {
                    mainHandler.post {
                        if (currentCoverUrl == url) {
                            currentCoverBitmap = bmp
                            updateSessionAndNotification(startInForeground = false)
                        } else {
                            bmp.recycle()
                        }
                    }
                }
            } catch (_: Throwable) {
                // Ignore cover fetch failures; notification stays visible without artwork
            } finally {
                conn?.disconnect()
            }
        }
    }

    private fun updatePlaybackState() {
        val session = mediaSession ?: return
        val actions = PlaybackState.ACTION_PLAY or
            PlaybackState.ACTION_PAUSE or
            PlaybackState.ACTION_PLAY_PAUSE or
            PlaybackState.ACTION_SKIP_TO_NEXT or
            PlaybackState.ACTION_SKIP_TO_PREVIOUS or
            (if (currentDurationMs > 0L) PlaybackState.ACTION_SEEK_TO else 0L)

        val state = PlaybackState.Builder()
            .setActions(actions)
            .setState(
                if (currentPlaying) PlaybackState.STATE_PLAYING else PlaybackState.STATE_PAUSED,
                currentPositionMs,
                if (currentPlaying) 1.0f else 0.0f
            )
            .build()
        session.setPlaybackState(state)
    }

    private fun updateSessionAndNotification(startInForeground: Boolean) {
        val session = mediaSession
        if (session != null) {
            val metaBuilder = MediaMetadata.Builder()
                .putString(MediaMetadata.METADATA_KEY_TITLE, currentTitle)
                .putString(MediaMetadata.METADATA_KEY_ARTIST, currentArtist)
            if (currentDurationMs > 0L) {
                metaBuilder.putLong(MediaMetadata.METADATA_KEY_DURATION, currentDurationMs)
            }
            currentCoverBitmap?.let {
                metaBuilder.putBitmap(MediaMetadata.METADATA_KEY_ALBUM_ART, it)
            }
            session.setMetadata(metaBuilder.build())
            updatePlaybackState()
        }

        val notification = buildNotification()
        if (startInForeground) {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                startForeground(
                    NOTIFICATION_ID,
                    notification,
                    ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK
                )
            } else {
                startForeground(NOTIFICATION_ID, notification)
            }
        } else {
            val manager = getSystemService(NotificationManager::class.java)
            manager?.notify(NOTIFICATION_ID, notification)
        }
    }

    private fun serviceActionIntent(action: String, requestCode: Int): PendingIntent {
        val intent = Intent(this, PlaybackForegroundService::class.java).apply {
            this.action = action
        }
        return PendingIntent.getService(
            this,
            requestCode,
            intent,
            PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT
        )
    }

    override fun onDestroy() {
        abandonAudioFocus()
        if (noisyReceiverRegistered) {
            try {
                unregisterReceiver(becomingNoisyReceiver)
            } catch (_: Throwable) {
                // Ignore if already unregistered
            }
            noisyReceiverRegistered = false
        }
        artExecutor.shutdownNow()
        mediaSession?.isActive = false
        mediaSession?.release()
        mediaSession = null
        if (wakeLock?.isHeld == true) {
            wakeLock?.release()
        }
        wakeLock = null
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    private fun createNotificationChannel() {
        val channel = NotificationChannel(
            CHANNEL_ID,
            "Music Playback",
            NotificationManager.IMPORTANCE_LOW
        ).apply {
            description = "Persistent media playback controls"
            setShowBadge(false)
            lockscreenVisibility = Notification.VISIBILITY_PUBLIC
        }
        val manager = getSystemService(NotificationManager::class.java)
        manager?.createNotificationChannel(channel)
    }

    private fun buildNotification(): Notification {
        val launchIntent = packageManager.getLaunchIntentForPackage(packageName)
        val contentIntent = PendingIntent.getActivity(
            this,
            0,
            launchIntent,
            PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT
        )

        val prevAction = Notification.Action.Builder(
            Icon.createWithResource(this, android.R.drawable.ic_media_previous),
            "Previous",
            serviceActionIntent(ACTION_PREV, 1)
        ).build()

        val toggleAction = Notification.Action.Builder(
            Icon.createWithResource(
                this,
                if (currentPlaying) android.R.drawable.ic_media_pause else android.R.drawable.ic_media_play
            ),
            if (currentPlaying) "Pause" else "Play",
            serviceActionIntent(ACTION_TOGGLE, 2)
        ).build()

        val nextAction = Notification.Action.Builder(
            Icon.createWithResource(this, android.R.drawable.ic_media_next),
            "Next",
            serviceActionIntent(ACTION_NEXT, 3)
        ).build()

        val style = Notification.MediaStyle()
            .setShowActionsInCompactView(0, 1, 2)
        mediaSession?.let { style.setMediaSession(it.sessionToken) }

        val builder = Notification.Builder(this, CHANNEL_ID)
            .setContentTitle(currentTitle)
            .setContentText(currentArtist)
            .setSmallIcon(
                if (currentPlaying) android.R.drawable.ic_media_play else android.R.drawable.ic_media_pause
            )
            .setContentIntent(contentIntent)
            .setVisibility(Notification.VISIBILITY_PUBLIC)
            .setOngoing(currentPlaying)
            .setOnlyAlertOnce(true)
            .addAction(prevAction)
            .addAction(toggleAction)
            .addAction(nextAction)
            .setStyle(style)

        currentCoverBitmap?.let { builder.setLargeIcon(it) }
        return builder.build()
    }
}
