use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{self, Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::Instant;

use crate::backend::runtime;
use crate::jam::{JamCommand, JamRequest, JamSource};
use crate::models::Track;

const MIN_PUBLISH_INTERVAL: Duration = Duration::from_secs(5);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10 * 60);
const MAX_STATE_AGE_MS: u64 = 20 * 60 * 1000;
const MAX_MESSAGE_BYTES: usize = 4096;
const MAX_FRAME_BYTES: usize = 32 * 1024;
const SIGNING_CONTEXT: &[u8] = b"GyroTube SPIN state v2\0";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RemoteRole {
    #[default]
    Idle,
    Host(String),
    Guest(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSyncPayload {
    pub track: Option<Track>,
    pub position_secs: f64,
    pub is_playing: bool,
    pub sent_at_ms: u64,
    pub allow_controls: bool,
    pub source_name: String,
    #[serde(default)]
    pub host_name: String,
}

impl RemoteSyncPayload {
    pub fn position_at(&self, now: u64) -> f64 {
        self.position_secs
            + if self.is_playing {
                now.saturating_sub(self.sent_at_ms) as f64 / 1000.0
            } else {
                0.0
            }
    }
}

pub struct RemoteUpdate {
    pub target: String,
    pub result: Result<RemoteSyncPayload, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum RemoteGuestEvent {
    AddTrack(Track),
    Control(String),
}

#[derive(Clone)]
struct Invitation {
    public_key: [u8; 32],
    command_token: [u8; 16],
}

impl Invitation {
    fn parse(code: &str) -> Result<Self, String> {
        let (public, token) = code.split_once('.').ok_or_else(|| {
            String::from("Invitación SPIN antigua o inválida. Pide una nueva al anfitrión.")
        })?;
        if public.len() != 43 || token.len() != 22 {
            return Err(String::from("Invitación SPIN inválida."));
        }
        let public_key = URL_SAFE_NO_PAD
            .decode(public)
            .ok()
            .and_then(|v| v.try_into().ok());
        let command_token = URL_SAFE_NO_PAD
            .decode(token)
            .ok()
            .and_then(|v| v.try_into().ok());
        let (Some(public_key), Some(command_token)) = (public_key, command_token) else {
            return Err(String::from("Invitación SPIN inválida."));
        };
        Ok(Self {
            public_key,
            command_token,
        })
    }

    fn code(&self) -> String {
        format!(
            "{}.{}",
            URL_SAFE_NO_PAD.encode(self.public_key),
            URL_SAFE_NO_PAD.encode(self.command_token)
        )
    }

    fn state_topic(&self) -> String {
        format!(
            "https://ntfy.sh/gyrotube-spin-{}",
            URL_SAFE_NO_PAD.encode(self.public_key)
        )
    }

    fn command_topic(&self) -> String {
        format!(
            "https://ntfy.sh/gyrotube-spin-cmd-{}",
            URL_SAFE_NO_PAD.encode(self.command_token)
        )
    }
}

fn new_identity() -> Result<(Invitation, Ed25519KeyPair), String> {
    let rng = SystemRandom::new();
    let document = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|_| String::from("No se pudo crear la clave de SPIN."))?;
    let key = Ed25519KeyPair::from_pkcs8(document.as_ref())
        .map_err(|_| String::from("No se pudo cargar la clave de SPIN."))?;
    let mut command_token = [0; 16];
    rng.fill(&mut command_token)
        .map_err(|_| String::from("No se pudo crear la invitación SPIN."))?;
    let public_key = key
        .public_key()
        .as_ref()
        .try_into()
        .map_err(|_| String::from("Clave pública SPIN inválida."))?;
    Ok((
        Invitation {
            public_key,
            command_token,
        },
        key,
    ))
}

#[derive(Serialize, Deserialize)]
struct SignedState {
    sequence: u64,
    payload: String,
    signature: String,
}

fn signing_bytes(invite: &Invitation, sequence: u64, payload: &str) -> Vec<u8> {
    [
        SIGNING_CONTEXT,
        &invite.command_token,
        &sequence.to_be_bytes(),
        payload.as_bytes(),
    ]
    .concat()
}

fn sign_state(
    key: &Ed25519KeyPair,
    invite: &Invitation,
    sequence: u64,
    payload: &RemoteSyncPayload,
) -> Result<String, String> {
    let payload =
        serde_json::to_string(payload).map_err(|e| format!("Estado SPIN inválido: {e}"))?;
    let signature = key.sign(&signing_bytes(invite, sequence, &payload));
    serde_json::to_string(&SignedState {
        sequence,
        payload,
        signature: URL_SAFE_NO_PAD.encode(signature.as_ref()),
    })
    .map_err(|e| format!("No se pudo firmar el estado SPIN: {e}"))
}

fn verify_state(
    invite: &Invitation,
    raw: &str,
    last_sequence: &mut u64,
    now: u64,
) -> Option<RemoteSyncPayload> {
    if raw.len() > MAX_MESSAGE_BYTES {
        return None;
    }
    let signed: SignedState = serde_json::from_str(raw).ok()?;
    if signed.sequence <= *last_sequence {
        return None;
    }
    let signature = URL_SAFE_NO_PAD.decode(&signed.signature).ok()?;
    signature::UnparsedPublicKey::new(&signature::ED25519, invite.public_key)
        .verify(
            &signing_bytes(invite, signed.sequence, &signed.payload),
            &signature,
        )
        .ok()?;
    let payload: RemoteSyncPayload = serde_json::from_str(&signed.payload).ok()?;
    if !payload.position_secs.is_finite()
        || payload.position_secs < 0.0
        || payload.sent_at_ms > now.saturating_add(30_000)
        || now.saturating_sub(payload.sent_at_ms) > MAX_STATE_AGE_MS
    {
        return None;
    }
    *last_sequence = signed.sequence;
    Some(payload)
}

pub fn normalize_room_code(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        let url =
            reqwest::Url::parse(trimmed).map_err(|_| String::from("URL de anfitrión inválida."))?;
        if url.host_str().is_none()
            || url.path() != "/"
            || url.query().is_some()
            || url.fragment().is_some()
            || !url.username().is_empty()
            || url.password().is_some()
        {
            return Err(String::from(
                "Usa la URL raíz del anfitrión, sin credenciales ni parámetros.",
            ));
        }
        return Ok(trimmed.trim_end_matches('/').to_string());
    }
    let code = trimmed
        .strip_prefix("SPIN-")
        .or_else(|| trimmed.strip_prefix("spin-"))
        .unwrap_or(trimmed);
    Ok(Invitation::parse(code)?.code())
}

pub fn local_display_name() -> String {
    std::env::var("USER")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| String::from("Host"))
}

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn client() -> reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT
        .get_or_init(|| {
            reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .build()
                .expect("SPIN HTTP client")
        })
        .clone()
}

struct PublishFailure {
    delay: Duration,
    message: String,
}

fn response_failure(
    status: reqwest::StatusCode,
    retry_after: Option<&str>,
) -> Option<PublishFailure> {
    if status.is_success() {
        return None;
    }
    // shortcut: parse numeric ntfy Retry-After; add HTTP-date support if the service starts returning dates.
    let delay = Duration::from_secs(
        retry_after
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(60)
            .clamp(5, 86_400),
    );
    let message = if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        format!(
            "SPIN: cuota del servicio agotada. Espera {} segundos.",
            delay.as_secs()
        )
    } else {
        format!("SPIN: el servicio rechazó la petición (HTTP {status}).")
    };
    Some(PublishFailure { delay, message })
}

async fn post_message(url: &str, body: String) -> Result<(), PublishFailure> {
    if body.len() > MAX_MESSAGE_BYTES {
        return Err(PublishFailure {
            delay: HEARTBEAT_INTERVAL,
            message: String::from("SPIN: los metadatos superan el límite de tamaño del servicio."),
        });
    }
    let response = client()
        .post(url)
        .timeout(Duration::from_secs(6))
        .body(body)
        .send()
        .await
        .map_err(|e| PublishFailure {
            delay: Duration::from_secs(10),
            message: format!("SPIN: no se pudo enviar la petición: {}", e.without_url()),
        })?;
    match response_failure(
        response.status(),
        response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|v| v.to_str().ok()),
    ) {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

fn state_changed(previous: Option<&RemoteSyncPayload>, next: &RemoteSyncPayload) -> bool {
    let Some(old) = previous else {
        return true;
    };
    let expected = old.position_secs
        + if old.is_playing {
            next.sent_at_ms.saturating_sub(old.sent_at_ms) as f64 / 1000.0
        } else {
            0.0
        };
    old.track.as_ref().map(|t| &t.id) != next.track.as_ref().map(|t| &t.id)
        || old.is_playing != next.is_playing
        || old.allow_controls != next.allow_controls
        || old.source_name != next.source_name
        || old.host_name != next.host_name
        || (expected - next.position_secs).abs() > 1.0
}

pub struct HostPublisher {
    pub invitation: String,
    sender: watch::Sender<Option<RemoteSyncPayload>>,
    task: JoinHandle<()>,
}

impl HostPublisher {
    pub fn new(updates: async_channel::Sender<RemoteUpdate>) -> Result<Self, String> {
        let (invite, key) = new_identity()?;
        let invitation = invite.code();
        let (sender, receiver) = watch::channel(None);
        let task = runtime().spawn(run_publisher(invite, key, receiver, updates));
        Ok(Self {
            invitation,
            sender,
            task,
        })
    }

    pub fn publish(&self, payload: RemoteSyncPayload) {
        self.sender.send_replace(Some(payload));
    }
}

impl Drop for HostPublisher {
    fn drop(&mut self) {
        self.task.abort();
    }
}

async fn run_publisher(
    invite: Invitation,
    key: Ed25519KeyPair,
    mut receiver: watch::Receiver<Option<RemoteSyncPayload>>,
    updates: async_channel::Sender<RemoteUpdate>,
) {
    let mut sent = None;
    let mut sequence = 0;
    let mut next_attempt = Instant::now();
    let mut heartbeat = Instant::now() + HEARTBEAT_INTERVAL;
    let mut retry = false;
    let mut reported = false;
    loop {
        if !retry {
            tokio::select! {
                changed = receiver.changed() => if changed.is_err() { break; },
                _ = tokio::time::sleep_until(heartbeat) => {},
            }
        }
        tokio::time::sleep_until(next_attempt).await;
        let Some(mut payload) = receiver.borrow_and_update().clone() else {
            heartbeat = Instant::now() + HEARTBEAT_INTERVAL;
            continue;
        };
        if !retry && Instant::now() < heartbeat && !state_changed(sent.as_ref(), &payload) {
            continue;
        }
        if payload.is_playing {
            payload.position_secs += now_ms().saturating_sub(payload.sent_at_ms) as f64 / 1000.0;
        }
        payload.sent_at_ms = now_ms();
        // Guests resolve their own catalog metadata; large cover URLs need not cross the relay.
        if let Some(track) = &mut payload.track {
            track.cover_url = None;
            track.cover_large_url = None;
        }
        sequence += 1;
        let result = match sign_state(&key, &invite, sequence, &payload) {
            Ok(body) => post_message(&invite.state_topic(), body).await,
            Err(message) => Err(PublishFailure {
                delay: HEARTBEAT_INTERVAL,
                message,
            }),
        };
        match result {
            Ok(()) => {
                sent = Some(payload);
                next_attempt = Instant::now() + MIN_PUBLISH_INTERVAL;
                heartbeat = Instant::now() + HEARTBEAT_INTERVAL;
                retry = false;
                reported = false;
            }
            Err(err) => {
                next_attempt = Instant::now() + err.delay;
                retry = true;
                if !reported {
                    let _ = updates
                        .send(RemoteUpdate {
                            target: invite.code(),
                            result: Err(err.message),
                        })
                        .await;
                    reported = true;
                }
            }
        }
    }
}

pub fn send_guest_event(
    target: String,
    event: RemoteGuestEvent,
    updates: async_channel::Sender<RemoteUpdate>,
) {
    runtime().spawn(async move {
        let result = if target.starts_with("http://") || target.starts_with("https://") {
            let (path, body) = match event {
                RemoteGuestEvent::AddTrack(track) => (
                    "/api/queue/add",
                    serde_json::to_string(&serde_json::json!({"id":track.id})),
                ),
                RemoteGuestEvent::Control(action) => (
                    "/api/control",
                    serde_json::to_string(&serde_json::json!({"action":action})),
                ),
            };
            match body {
                Ok(body) => post_message(&format!("{target}{path}"), body)
                    .await
                    .map_err(|err| err.message),
                Err(err) => Err(err.to_string()),
            }
        } else {
            match (Invitation::parse(&target), serde_json::to_string(&event)) {
                (Ok(invite), Ok(body)) => post_message(&invite.command_topic(), body)
                    .await
                    .map_err(|err| err.message),
                _ => Err(String::from("Invitación o comando SPIN inválido.")),
            }
        };
        if let Err(message) = result {
            let _ = updates
                .send(RemoteUpdate {
                    target,
                    result: Err(message),
                })
                .await;
        }
    });
}

#[derive(Deserialize)]
struct NtfyMsg {
    event: Option<String>,
    message: Option<String>,
}

fn take_frame(buffer: &mut Vec<u8>) -> Result<Option<NtfyMsg>, String> {
    let Some(end) = buffer.iter().position(|b| *b == b'\n') else {
        return if buffer.len() > MAX_FRAME_BYTES {
            Err(String::from("SPIN: respuesta demasiado grande."))
        } else {
            Ok(None)
        };
    };
    if end > MAX_FRAME_BYTES {
        return Err(String::from("SPIN: respuesta demasiado grande."));
    }
    let line: Vec<u8> = buffer.drain(..=end).collect();
    serde_json::from_slice(&line)
        .map(Some)
        .map_err(|_| String::from("SPIN: respuesta del servicio inválida."))
}

async fn next_message(
    response: &mut reqwest::Response,
    buffer: &mut Vec<u8>,
) -> Result<Option<String>, String> {
    loop {
        if let Some(message) = take_frame(buffer)? {
            if message.event.as_deref() == Some("message")
                && let Some(raw) = message.message
            {
                return Ok(Some(raw));
            }
            continue;
        }
        let chunk = tokio::time::timeout(Duration::from_secs(90), response.chunk())
            .await
            .map_err(|_| String::from("SPIN: la conexión dejó de responder."))?
            .map_err(|_| String::from("SPIN: conexión interrumpida."))?;
        match chunk {
            Some(bytes) => buffer.extend_from_slice(&bytes),
            None => return Ok(None),
        }
    }
}

async fn open_stream(url: &str) -> Result<reqwest::Response, PublishFailure> {
    let response = tokio::time::timeout(Duration::from_secs(10), client().get(url).send())
        .await
        .map_err(|_| PublishFailure {
            delay: Duration::from_secs(10),
            message: String::from("SPIN: tiempo de conexión agotado."),
        })?
        .map_err(|e| PublishFailure {
            delay: Duration::from_secs(10),
            message: format!("SPIN: no se pudo conectar: {}", e.without_url()),
        })?;
    match response_failure(
        response.status(),
        response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|v| v.to_str().ok()),
    ) {
        Some(err) => Err(err),
        None => Ok(response),
    }
}

pub fn start_host_cmd_listener(
    room: String,
    cmd_tx: async_channel::Sender<JamRequest>,
    updates: async_channel::Sender<RemoteUpdate>,
) -> JoinHandle<()> {
    runtime().spawn(async move {
        let Ok(invite) = Invitation::parse(&room) else {
            return;
        };
        let mut reported = false;
        loop {
            let result = async {
                let mut response = open_stream(&format!("{}/json", invite.command_topic())).await?;
                let mut buffer = Vec::new();
                while let Some(raw) =
                    next_message(&mut response, &mut buffer)
                        .await
                        .map_err(|message| PublishFailure {
                            delay: Duration::from_secs(5),
                            message,
                        })?
                {
                    let Ok(event) = serde_json::from_str::<RemoteGuestEvent>(&raw) else {
                        continue;
                    };
                    let command = match event {
                        RemoteGuestEvent::AddTrack(t) => {
                            Some(JamCommand::AddTrack(t.id, String::from("remoto")))
                        }
                        RemoteGuestEvent::Control(action) => match action.as_str() {
                            "play_pause" => Some(JamCommand::TogglePlayPause),
                            "next" => Some(JamCommand::NextTrack),
                            "prev" => Some(JamCommand::PrevTrack),
                            _ => None,
                        },
                    };
                    if let Some(command) = command
                        && cmd_tx
                            .send(JamRequest {
                                source: JamSource::Remote(room.clone()),
                                command,
                            })
                            .await
                            .is_err()
                    {
                        return Ok(());
                    }
                    reported = false;
                }
                Err(PublishFailure {
                    delay: Duration::from_secs(5),
                    message: String::from("SPIN: conexión de invitados cerrada."),
                })
            }
            .await;
            match result {
                Ok(()) => break,
                Err(err) => {
                    if !reported {
                        let _ = updates
                            .send(RemoteUpdate {
                                target: room.clone(),
                                result: Err(err.message),
                            })
                            .await;
                        reported = true;
                    }
                    tokio::time::sleep(err.delay).await;
                }
            }
        }
    })
}

async fn latest_cached_state(
    invite: &Invitation,
    last_sequence: &mut u64,
) -> Result<Option<RemoteSyncPayload>, PublishFailure> {
    let mut response = open_stream(&format!(
        "{}/json?poll=1&since={}s",
        invite.state_topic(),
        MAX_STATE_AGE_MS / 1000
    ))
    .await?;
    let mut candidate_sequence = *last_sequence;
    let latest = tokio::time::timeout(Duration::from_secs(10), async {
        let mut buffer = Vec::new();
        let mut latest = None;
        while let Some(raw) = next_message(&mut response, &mut buffer).await? {
            if let Some(payload) = verify_state(invite, &raw, &mut candidate_sequence, now_ms()) {
                latest = Some(payload);
            }
        }
        Ok::<_, String>(latest)
    })
    .await
    .map_err(|_| PublishFailure {
        delay: Duration::from_secs(10),
        message: String::from("SPIN: tiempo de lectura del estado agotado."),
    })?
    .map_err(|message| PublishFailure {
        delay: Duration::from_secs(5),
        message,
    })?;
    *last_sequence = candidate_sequence;
    Ok(latest)
}

pub fn start_guest_sync(
    target: String,
    ui_tx: async_channel::Sender<RemoteUpdate>,
) -> JoinHandle<()> {
    runtime().spawn(async move {
        if target.starts_with("http://") || target.starts_with("https://") {
            #[derive(Deserialize)]
            struct LocalStateResp {
                allow_controls: bool,
                is_playing: bool,
                position_secs: f64,
                now_playing: Option<crate::jam::JamTrackInfo>,
            }
            let mut reported = false;
            loop {
                let result: Result<RemoteSyncPayload, PublishFailure> = async {
                    let response = open_stream(&format!("{target}/api/state")).await?;
                    let st = tokio::time::timeout(
                        Duration::from_secs(3),
                        response.json::<LocalStateResp>(),
                    )
                    .await
                    .ok()
                    .and_then(Result::ok)
                    .ok_or_else(|| PublishFailure {
                        delay: Duration::from_secs(3),
                        message: String::from("SPIN: estado del anfitrión inválido."),
                    })?;
                    let track = st.now_playing.map(|n| Track {
                        id: n.id,
                        title: n.title,
                        artist: n.artist,
                        album: n.album,
                        duration_secs: n.duration_secs,
                        cover_url: n.cover_url.clone(),
                        cover_large_url: n.cover_url,
                        is_music: false,
                    });
                    Ok(RemoteSyncPayload {
                        track,
                        position_secs: st.position_secs,
                        is_playing: st.is_playing,
                        sent_at_ms: now_ms(),
                        allow_controls: st.allow_controls,
                        source_name: String::from("SPIN"),
                        host_name: String::new(),
                    })
                }
                .await;
                match result {
                    Ok(payload) => {
                        if ui_tx
                            .send(RemoteUpdate {
                                target: target.clone(),
                                result: Ok(payload),
                            })
                            .await
                            .is_err()
                        {
                            break;
                        }
                        reported = false;
                        tokio::time::sleep(Duration::from_millis(1500)).await;
                    }
                    Err(err) => {
                        if !reported {
                            let _ = ui_tx
                                .send(RemoteUpdate {
                                    target: target.clone(),
                                    result: Err(err.message),
                                })
                                .await;
                            reported = true;
                        }
                        tokio::time::sleep(err.delay).await;
                    }
                }
            }
            return;
        }
        let Ok(invite) = Invitation::parse(&target) else {
            return;
        };
        let mut last_sequence = 0;
        let mut reported = false;
        loop {
            let result = async {
                if let Some(payload) = latest_cached_state(&invite, &mut last_sequence).await? {
                    if ui_tx
                        .send(RemoteUpdate {
                            target: target.clone(),
                            result: Ok(payload),
                        })
                        .await
                        .is_err()
                    {
                        return Ok(());
                    }
                    reported = false;
                }
                let mut response =
                    open_stream(&format!("{}/json?since=latest", invite.state_topic())).await?;
                let mut buffer = Vec::new();
                while let Some(raw) =
                    next_message(&mut response, &mut buffer)
                        .await
                        .map_err(|message| PublishFailure {
                            delay: Duration::from_secs(5),
                            message,
                        })?
                {
                    if let Some(payload) = verify_state(&invite, &raw, &mut last_sequence, now_ms())
                    {
                        if ui_tx
                            .send(RemoteUpdate {
                                target: target.clone(),
                                result: Ok(payload),
                            })
                            .await
                            .is_err()
                        {
                            return Ok(());
                        }
                        reported = false;
                    }
                }
                Err(PublishFailure {
                    delay: Duration::from_secs(5),
                    message: String::from("SPIN: conexión con el anfitrión cerrada."),
                })
            }
            .await;
            match result {
                Ok(()) => break,
                Err(err) => {
                    if !reported {
                        let _ = ui_tx
                            .send(RemoteUpdate {
                                target: target.clone(),
                                result: Err(err.message),
                            })
                            .await;
                        reported = true;
                    }
                    tokio::time::sleep(err.delay).await;
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(now: u64) -> RemoteSyncPayload {
        RemoteSyncPayload {
            track: None,
            position_secs: 12.5,
            is_playing: true,
            sent_at_ms: now,
            allow_controls: false,
            source_name: "Album".into(),
            host_name: "Host".into(),
        }
    }

    #[test]
    fn invitations_use_os_random_keys_and_do_not_change_case() {
        let (invite, key) = new_identity().unwrap();
        let (other, _) = new_identity().unwrap();
        assert_ne!(invite.code(), other.code());
        assert_eq!(invite.public_key.as_slice(), key.public_key().as_ref());
        let code = invite.code();
        assert_eq!(
            normalize_room_code(&format!(" SPIN-{code} ")).unwrap(),
            code
        );
        assert_eq!(normalize_room_code(&format!("spin-{code}")).unwrap(), code);
        assert!(normalize_room_code("spin-a7k9m2").is_err());
        assert!(normalize_room_code("fake.invalid").is_err());
        assert!(invite.state_topic().rsplit('/').next().unwrap().len() <= 64);
        assert!(invite.command_topic().rsplit('/').next().unwrap().len() <= 64);
        assert_eq!(
            normalize_room_code("http://192.168.1.10:8765/").unwrap(),
            "http://192.168.1.10:8765"
        );
        assert!(normalize_room_code("http://user:secret@localhost/").is_err());
        assert!(normalize_room_code("http://localhost/path").is_err());
    }

    #[test]
    fn only_the_pinned_host_can_sign_state_and_replay_cannot_advance_sequence() {
        let (invite, key) = new_identity().unwrap();
        let (_, guest_key) = new_identity().unwrap();
        let now = 1_000_000;
        let state = payload(now);
        let mut last = 0;
        let raw = sign_state(&key, &invite, 1, &state).unwrap();
        assert!(verify_state(&invite, &raw, &mut last, now).is_some());
        assert_eq!(last, 1);
        let mut changed_invite = invite.clone();
        changed_invite.command_token[0] ^= 1;
        assert!(verify_state(&changed_invite, &raw, &mut 0, now).is_none());
        assert!(verify_state(&invite, &raw, &mut last, now).is_none());
        let forgery = sign_state(&guest_key, &invite, u64::MAX, &state).unwrap();
        assert!(verify_state(&invite, &forgery, &mut last, now).is_none());
        assert_eq!(last, 1);
        let mut altered: SignedState = serde_json::from_str(&raw).unwrap();
        altered.sequence = u64::MAX;
        assert!(
            verify_state(
                &invite,
                &serde_json::to_string(&altered).unwrap(),
                &mut last,
                now
            )
            .is_none()
        );
        altered = serde_json::from_str(&sign_state(&key, &invite, 2, &state).unwrap()).unwrap();
        altered.payload = serde_json::to_string(&payload(now + 1)).unwrap();
        assert!(
            verify_state(
                &invite,
                &serde_json::to_string(&altered).unwrap(),
                &mut last,
                now
            )
            .is_none()
        );
        assert!(
            verify_state(
                &invite,
                &serde_json::to_string(&state).unwrap(),
                &mut last,
                now
            )
            .is_none()
        );
        let next = sign_state(&key, &invite, 2, &state).unwrap();
        assert!(verify_state(&invite, &next, &mut last, now).is_some());
        assert_eq!(last, 2);
    }

    #[test]
    fn replaying_an_older_cached_state_does_not_replace_a_newer_signed_state() {
        let (invite, key) = new_identity().unwrap();
        let now = 1_000_000;
        let first = payload(now - 10_000);
        let newer = RemoteSyncPayload {
            position_secs: 30.0,
            ..payload(now)
        };
        let frames = [
            sign_state(&key, &invite, 1, &first).unwrap(),
            sign_state(&key, &invite, 2, &newer).unwrap(),
            sign_state(&key, &invite, 1, &first).unwrap(),
        ];
        let mut sequence = 0;
        let mut latest = None;
        for raw in frames {
            if let Some(payload) = verify_state(&invite, &raw, &mut sequence, now) {
                latest = Some(payload);
            }
        }
        assert_eq!(sequence, 2);
        assert_eq!(latest.unwrap().position_secs, 30.0);
    }

    #[test]
    fn expired_future_or_invalid_positions_are_not_authenticated_state() {
        let (invite, key) = new_identity().unwrap();
        let now = 2_000_000;
        for state in [
            payload(now - MAX_STATE_AGE_MS - 1),
            payload(now + 30_001),
            RemoteSyncPayload {
                position_secs: -1.0,
                ..payload(now)
            },
        ] {
            let mut last = 0;
            assert!(
                verify_state(
                    &invite,
                    &sign_state(&key, &invite, 1, &state).unwrap(),
                    &mut last,
                    now
                )
                .is_none()
            );
            assert_eq!(last, 0);
        }
    }

    #[test]
    fn regular_playback_ticks_do_not_publish_but_actions_do() {
        let before = payload(1_000);
        let mut next = payload(11_000);
        next.position_secs += 10.0;
        assert!(!state_changed(Some(&before), &next));
        next.position_secs += 5.0;
        assert!(state_changed(Some(&before), &next));
        next.position_secs -= 5.0;
        next.is_playing = false;
        assert!(state_changed(Some(&before), &next));
        next.is_playing = true;
        next.allow_controls = true;
        assert!(state_changed(Some(&before), &next));
        assert!(state_changed(None, &next));
    }

    #[test]
    fn receiving_a_cached_message_extrapolates_playback_but_keeps_pause() {
        let state = payload(1_000);
        assert_eq!(state.position_at(121_000), 132.5);
        let paused = RemoteSyncPayload {
            is_playing: false,
            ..state
        };
        assert_eq!(paused.position_at(121_000), 12.5);
    }

    #[test]
    fn resolving_catalog_covers_does_not_publish_a_new_state() {
        let track: Track = serde_json::from_value(serde_json::json!({
            "id":"audio123456", "title":"Song", "artist":"Artist"
        }))
        .unwrap();
        let old = RemoteSyncPayload {
            track: Some(track.clone()),
            ..payload(1_000)
        };
        let mut track_with_cover = track;
        track_with_cover.cover_url = Some("https://example.com/cover".into());
        let next = RemoteSyncPayload {
            track: Some(track_with_cover),
            position_secs: 22.5,
            ..payload(11_000)
        };
        assert!(!state_changed(Some(&old), &next));
    }

    #[test]
    fn rate_limits_respect_retry_after_and_bound_untrusted_delays() {
        let status = reqwest::StatusCode::TOO_MANY_REQUESTS;
        assert_eq!(
            response_failure(status, Some("120")).unwrap().delay,
            Duration::from_secs(120)
        );
        assert_eq!(
            response_failure(status, Some("0")).unwrap().delay,
            MIN_PUBLISH_INTERVAL
        );
        assert_eq!(
            response_failure(status, Some("18446744073709551615"))
                .unwrap()
                .delay,
            Duration::from_secs(86_400)
        );
        assert_eq!(
            response_failure(status, None).unwrap().delay,
            Duration::from_secs(60)
        );
        assert!(response_failure(reqwest::StatusCode::OK, None).is_none());
    }

    #[test]
    fn streamed_json_preserves_split_utf8_and_rejects_oversized_frames() {
        let raw = "{\"event\":\"message\",\"message\":\"canción\"}\n".as_bytes();
        let split = raw.iter().position(|b| *b == 0xc3).unwrap() + 1;
        let mut buffer = raw[..split].to_vec();
        assert!(take_frame(&mut buffer).unwrap().is_none());
        buffer.extend_from_slice(&raw[split..]);
        assert_eq!(
            take_frame(&mut buffer).unwrap().unwrap().message.as_deref(),
            Some("canción")
        );
        let mut oversized = vec![b'x'; MAX_FRAME_BYTES + 1];
        assert!(take_frame(&mut oversized).is_err());
    }

    #[tokio::test]
    async fn dropping_host_aborts_the_idle_publisher_without_network_calls() {
        let (updates, _) = async_channel::unbounded();
        let publisher = HostPublisher::new(updates).unwrap();
        let task = publisher.task.abort_handle();
        drop(publisher);
        tokio::task::yield_now().await;
        // The publisher uses the shared runtime, so join its aborted task through a separate handle.
        for _ in 0..20 {
            if task.is_finished() {
                return;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        assert!(task.is_finished());
    }
}
