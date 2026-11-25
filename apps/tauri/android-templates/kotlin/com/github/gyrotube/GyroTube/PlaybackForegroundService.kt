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
    private val mainHandler = Handler(Looper.getMainLooper())
    private val artExecutor = Executors.newSingleThreadExecutor()

    private var currentTitle = "GyroTube"
    private var currentArtist = ""
    private var currentPlaying = false
    private var currentPositionMs = 0L
    private var currentDurationMs = 0L
    private var currentCoverUrl: String? = null
    private var currentCoverBitmap: Bitmap? = null
    private var noisyReceiverRegistered = false
    private var audioManager: AudioManager? = null
    private var audioFocusRequest: AudioFocusRequest? = null
    private var hasAudioFocus = false
    private var isDucked = false

    private val audioFocusListener = AudioManager.OnAudioFocusChangeListener { focusChange ->
        when (focusChange) {
            AudioManager.AUDIOFOCUS_LOSS,
            AudioManager.AUDIOFOCUS_LOSS_TRANSIENT -> {
                hasAudioFocus = false
                isDucked = false
                if (currentPlaying) {
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
            }
        }
    }

    private fun requestMediaAudioFocus(): Boolean {
        val am = audioManager ?: return true
        if (hasAudioFocus) return true
        val result = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val attrs = AudioAttributes.Builder()
                .setUsage(AudioAttributes.USAGE_MEDIA)
                .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC)
                .build()
            val req = AudioFocusRequest.Builder(AudioManager.AUDIOFOCUS_GAIN)
                .setAudioAttributes(attrs)
                .setAcceptsDelayedFocusGain(false)
                .setOnAudioFocusChangeListener(audioFocusListener, mainHandler)
                .build()
            audioFocusRequest = req
            am.requestAudioFocus(req)
        } else {
            @Suppress("DEPRECATION")
            am.requestAudioFocus(
                audioFocusListener,
                AudioManager.STREAM_MUSIC,
                AudioManager.AUDIOFOCUS_GAIN
            )
        }
        hasAudioFocus = (result == AudioManager.AUDIOFOCUS_REQUEST_GRANTED)
        return hasAudioFocus
    }

    private fun abandonMediaAudioFocus() {
        val am = audioManager ?: return
        if (!hasAudioFocus && audioFocusRequest == null) return
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            audioFocusRequest?.let { am.abandonAudioFocusRequest(it) }
            audioFocusRequest = null
        } else {
            @Suppress("DEPRECATION")
            am.abandonAudioFocus(audioFocusListener)
        }
        hasAudioFocus = false
        isDucked = false
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
        audioManager = getSystemService(AudioManager::class.java)
        val pm = getSystemService(PowerManager::class.java)
        wakeLock = pm?.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "GyroTube::PlaybackWakeLock")?.apply {
            setReferenceCounted(false)
        }
        mediaSession = MediaSession(this, "GyroTubeSession").apply {
            setCallback(object : MediaSession.Callback() {
                override fun onPlay() {
                    onMediaAction?.invoke("play")
                }

                override fun onPause() {
                    onMediaAction?.invoke("pause")
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
            currentTitle = intent.getStringExtra(EXTRA_TITLE)?.takeIf { it.isNotBlank() } ?: "GyroTube"
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

        updateSessionAndNotification(startInForeground = true)

        if (currentPlaying) {
            requestMediaAudioFocus()
            if (wakeLock?.isHeld == false) {
                wakeLock?.acquire(4 * 60 * 60 * 1000L)
            }
        } else {
            abandonMediaAudioFocus()
            if (wakeLock?.isHeld == true) {
                wakeLock?.release()
            }
        }

        return START_STICKY
    }

    private fun fetchCoverArtAsync(url: String) {
        artExecutor.execute {
            try {
                val conn = (URL(url).openConnection() as HttpURLConnection).apply {
                    connectTimeout = 4000
                    readTimeout = 4000
                    setRequestProperty("User-Agent", "GyroTube/0.1 (Android)")
                }
                val bmp = conn.inputStream.use { BitmapFactory.decodeStream(it) }
                conn.disconnect()
                if (bmp != null) {
                    mainHandler.post {
                        if (currentCoverUrl == url) {
                            currentCoverBitmap = bmp
                            updateSessionAndNotification(startInForeground = false)
                        }
                    }
                }
            } catch (_: Throwable) {
                // Ignore cover fetch failures; notification stays visible without artwork
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
        abandonMediaAudioFocus()
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
