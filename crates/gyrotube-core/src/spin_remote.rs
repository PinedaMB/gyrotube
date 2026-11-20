use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Result, bail, ensure};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use iroh::{
    Endpoint, EndpointAddr,
    endpoint::{Connection, RecvStream, SendStream, presets},
};
use iroh_tickets::{Ticket, endpoint::EndpointTicket};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{oneshot, watch},
    task::{JoinHandle, JoinSet},
    time::{Instant, timeout},
};

use crate::backend::runtime;
use crate::jam::{JamCommand, JamRequest, JamSource};
use crate::models::Track;

const ALPN: &[u8] = b"gyrotube/spin/1";
const IO_TIMEOUT: Duration = Duration::from_secs(8);
const MAX_FRAME: usize = 8192;
const MAX_GUESTS: usize = 16;

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

fn encode_ticket(addr: EndpointAddr, token: &[u8; 16]) -> String {
    format!(
        "{}.{}",
        EndpointTicket::new(addr).encode_string(),
        URL_SAFE_NO_PAD.encode(token)
    )
}

fn parse_ticket(code: &str) -> Result<(EndpointAddr, [u8; 16]), String> {
    let invalid = || "Invitación SPIN inválida. Pide un ticket GYRO nuevo.".to_string();
    if code.len() > 2048 {
        return Err(invalid());
    }
    let (ticket, token) = code.split_once('.').ok_or_else(invalid)?;
    let ticket: EndpointTicket = ticket.parse().map_err(|_| invalid())?;
    let token = URL_SAFE_NO_PAD
        .decode(token)
        .ok()
        .and_then(|v| v.try_into().ok())
        .ok_or_else(invalid)?;
    Ok((ticket.into(), token))
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
    let code = trimmed.strip_prefix("GYRO-").unwrap_or(trimmed);
    parse_ticket(code)?;
    Ok(code.to_owned())
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
            crate::tls::http_client_builder()
                .connect_timeout(Duration::from_secs(5))
                .build()
                .expect("SPIN HTTP client")
        })
        .clone()
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

async fn write_frame<T: Serialize>(send: &mut SendStream, message: &T) -> Result<()> {
    let bytes = serde_json::to_vec(message)?;
    ensure!(bytes.len() <= MAX_FRAME, "Mensaje SPIN demasiado grande");
    timeout(IO_TIMEOUT, async {
        send.write_u16(bytes.len() as u16).await?;
        send.write_all(&bytes).await?;
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    Ok(())
}

async fn read_frame<T: DeserializeOwned>(recv: &mut RecvStream) -> Result<T> {
    let len = recv.read_u16().await? as usize;
    ensure!(len <= MAX_FRAME, "Mensaje SPIN demasiado grande");
    let mut bytes = vec![0; len];
    recv.read_exact(&mut bytes).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub struct HostPublisher {
    pub invitation: String,
    sender: watch::Sender<Option<RemoteSyncPayload>>,
    task: JoinHandle<()>,
    endpoint: Endpoint,
}

impl HostPublisher {
    pub async fn new(commands: async_channel::Sender<JamRequest>) -> Result<Self> {
        let endpoint = Endpoint::builder(presets::N0)
            .alpns(vec![ALPN.to_vec()])
            .bind()
            .await?;
        // The local addresses in the ticket still work if the relay is unavailable.
        let _ = timeout(IO_TIMEOUT, endpoint.online()).await;
        Self::serve(endpoint, commands)
    }

    fn serve(endpoint: Endpoint, commands: async_channel::Sender<JamRequest>) -> Result<Self> {
        let mut token = [0; 16];
        SystemRandom::new()
            .fill(&mut token)
            .map_err(|_| anyhow::anyhow!("No se pudo generar el ticket SPIN"))?;
        let invitation = encode_ticket(endpoint.addr(), &token);
        let room = invitation.clone();
        let listener = endpoint.clone();
        let (sender, receiver) = watch::channel(None);
        let task = runtime().spawn(async move {
            let mut guests = JoinSet::new();
            loop {
                tokio::select! {
                    _ = guests.join_next(), if !guests.is_empty() => {}
                    incoming = listener.accept() => {
                        let Some(incoming) = incoming else { break };
                        if guests.len() >= MAX_GUESTS {
                            incoming.refuse();
                            continue;
                        }
                        let state = receiver.clone();
                        let commands = commands.clone();
                        let room = room.clone();
                        guests.spawn(async move {
                            if let Ok(Ok(connection)) = timeout(IO_TIMEOUT, incoming).await {
                                let _ = serve_guest(&connection, token, state, commands, room).await;
                                connection.close(0u32.into(), b"SPIN");
                            }
                        });
                    }
                }
            }
        });
        Ok(Self {
            invitation,
            sender,
            task,
            endpoint,
        })
    }

    pub fn publish(&self, payload: RemoteSyncPayload) {
        self.sender.send_replace(Some(payload));
    }
}

impl Drop for HostPublisher {
    fn drop(&mut self) {
        self.task.abort();
        let endpoint = self.endpoint.clone();
        runtime().spawn(async move { endpoint.close().await });
    }
}

async fn serve_guest(
    connection: &Connection,
    token: [u8; 16],
    mut state: watch::Receiver<Option<RemoteSyncPayload>>,
    commands: async_channel::Sender<JamRequest>,
    room: String,
) -> Result<()> {
    let (mut send, mut recv) = timeout(IO_TIMEOUT, async {
        let (send, mut recv) = connection.accept_bi().await?;
        let mut provided = [0; 16];
        recv.read_exact(&mut provided).await?;
        ensure!(provided == token, "Ticket SPIN inválido");
        Ok::<_, anyhow::Error>((send, recv))
    })
    .await??;

    tokio::select! {
        result = async {
            let mut last = None;
            let mut sent = Instant::now();
            loop {
                let snapshot = state.borrow_and_update().clone();
                if let Some(mut snapshot) = snapshot
                    && (state_changed(last.as_ref(), &snapshot) || sent.elapsed() >= Duration::from_secs(2))
                {
                    last = Some(snapshot.clone());
                    snapshot.position_secs = snapshot.position_at(now_ms());
                    snapshot.sent_at_ms = now_ms();
                    if let Some(track) = &mut snapshot.track {
                        track.cover_url = None;
                        track.cover_large_url = None;
                    }
                    write_frame(&mut send, &snapshot).await?;
                    sent = Instant::now();
                }
                tokio::select! {
                    _ = tokio::time::sleep_until(sent + Duration::from_secs(2)), if last.is_some() => {}
                    changed = state.changed() => changed?,
                }
            }
        } => result,
        result = async {
            loop {
                let event = read_frame::<RemoteGuestEvent>(&mut recv).await?;
                let command = match event {
                    RemoteGuestEvent::AddTrack(track) => JamCommand::AddTrack(track.id, "remoto".into()),
                    RemoteGuestEvent::Control(action) => match action.as_str() {
                        "play_pause" => JamCommand::TogglePlayPause,
                        "next" => JamCommand::NextTrack,
                        "prev" => JamCommand::PrevTrack,
                        _ => bail!("Control SPIN inválido"),
                    },
                };
                commands.try_send(JamRequest { source: JamSource::Remote(room.clone()), command })?;
                tokio::time::sleep(Duration::from_millis(150)).await;
            }
        } => result,
    }
}

pub struct GuestSession {
    _stop: oneshot::Sender<()>,
    events: async_channel::Sender<RemoteGuestEvent>,
    connected: Arc<AtomicBool>,
}

impl GuestSession {
    pub fn send(&self, event: RemoteGuestEvent) -> bool {
        self.connected.load(Ordering::Acquire) && self.events.try_send(event).is_ok()
    }
}

async fn receive_room(
    connection: &Connection,
    token: &[u8; 16],
    events: &async_channel::Receiver<RemoteGuestEvent>,
    target: &str,
    ui_tx: &async_channel::Sender<RemoteUpdate>,
    connected: &AtomicBool,
) -> Result<()> {
    let (mut send, mut recv) = timeout(IO_TIMEOUT, connection.open_bi()).await??;
    timeout(IO_TIMEOUT, send.write_all(token)).await??;
    tokio::select! {
        result = async {
            loop {
                let mut payload: RemoteSyncPayload = timeout(IO_TIMEOUT, read_frame(&mut recv)).await??;
                ensure!(payload.position_secs.is_finite() && payload.position_secs >= 0.0, "Posición SPIN inválida");
                // Use the receiver's clock after taking the current position from the host.
                payload.sent_at_ms = now_ms();
                ui_tx.send(RemoteUpdate { target: target.into(), result: Ok(payload) }).await?;
                connected.store(true, Ordering::Release);
            }
        } => result,
        result = async {
            while let Ok(event) = events.recv().await {
                write_frame(&mut send, &event).await?;
            }
            Ok(())
        } => result,
    }
}

pub fn start_guest_sync(
    target: String,
    ui_tx: async_channel::Sender<RemoteUpdate>,
) -> GuestSession {
    let (sender, events) = async_channel::bounded(8);
    let is_lan = target.starts_with("http://") || target.starts_with("https://");
    let connected = Arc::new(AtomicBool::new(is_lan));
    let ready = connected.clone();
    let (stop, stopped) = oneshot::channel();
    runtime().spawn(async move {
        if is_lan {
            tokio::select! {
                _ = stopped => {}
                _ = poll_lan(&target, &ui_tx) => {}
                _ = async {
                    while let Ok(event) = events.recv().await {
                        if publish_lan(&target, event).await.is_err() {
                            report_error(&ui_tx, &target, "SPIN: el anfitrión local rechazó el comando.").await;
                        }
                    }
                } => {}
            }
            return;
        }
        let Ok((addr, token)) = parse_ticket(&target) else { return };
        let Ok(endpoint) = Endpoint::bind(presets::N0).await else {
            report_error(&ui_tx, &target, "SPIN: no se pudo iniciar la conexión.").await;
            return;
        };
        tokio::select! {
            _ = stopped => {}
            _ = async {
                let mut reported = false;
                loop {
                    if let Ok(Ok(connection)) = timeout(IO_TIMEOUT, endpoint.connect(addr.clone(), ALPN)).await {
                        while events.try_recv().is_ok() {}
                        let _ = receive_room(&connection, &token, &events, &target, &ui_tx, &ready).await;
                        connection.close(0u32.into(), b"SPIN");
                        if ready.swap(false, Ordering::AcqRel) {
                            reported = false;
                        }
                    }
                    if !reported {
                        report_error(&ui_tx, &target, "SPIN: sala inaccesible. Verifica el ticket y que el anfitrión siga conectado.").await;
                        reported = true;
                    }
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            } => {}
        }
        endpoint.close().await;
    });
    GuestSession {
        _stop: stop,
        events: sender,
        connected,
    }
}

async fn report_error(ui_tx: &async_channel::Sender<RemoteUpdate>, target: &str, message: &str) {
    let _ = ui_tx
        .send(RemoteUpdate {
            target: target.into(),
            result: Err(message.into()),
        })
        .await;
}

async fn publish_lan(target: &str, event: RemoteGuestEvent) -> Result<()> {
    let (path, body) = match event {
        RemoteGuestEvent::AddTrack(track) => ("/api/queue/add", serde_json::json!({"id":track.id})),
        RemoteGuestEvent::Control(action) => ("/api/control", serde_json::json!({"action":action})),
    };
    client()
        .post(format!("{target}{path}"))
        .timeout(IO_TIMEOUT)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

async fn poll_lan(target: &str, ui_tx: &async_channel::Sender<RemoteUpdate>) {
    #[derive(Deserialize)]
    struct LocalStateResp {
        allow_controls: bool,
        is_playing: bool,
        position_secs: f64,
        now_playing: Option<crate::jam::JamTrackInfo>,
    }
    let mut reported = false;
    loop {
        let result: anyhow::Result<RemoteSyncPayload> = async {
            let st = client()
                .get(format!("{target}/api/state"))
                .timeout(IO_TIMEOUT)
                .send()
                .await?
                .error_for_status()?
                .json::<LocalStateResp>()
                .await?;
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
                        target: target.to_owned(),
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
            Err(_) => {
                if !reported {
                    let _ = ui_tx
                        .send(RemoteUpdate {
                            target: target.to_owned(),
                            result: Err("SPIN: no se pudo consultar el anfitrión local.".into()),
                        })
                        .await;
                    reported = true;
                }
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    }
}

pub fn ticket_qr_svg(ticket: &str) -> Option<String> {
    Some(
        qrcode::QrCode::new(ticket)
            .ok()?
            .render::<qrcode::render::svg::Color>()
            .min_dimensions(220, 220)
            .build(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ticket_authorizes_state_and_commands_until_host_closes() -> Result<()> {
        timeout(Duration::from_secs(20), async {
            let host = Endpoint::builder(presets::Minimal)
                .relay_mode(iroh::RelayMode::Disabled)
                .alpns(vec![ALPN.to_vec()])
                .bind()
                .await?;
            let addr = EndpointAddr::new(host.id()).with_ip_addr(std::net::SocketAddr::from((
                [127, 0, 0, 1],
                host.bound_sockets()[0].port(),
            )));
            let (commands, requests) = async_channel::bounded(4);
            let host = HostPublisher::serve(host, commands)?;
            let (_, token) = parse_ticket(&host.invitation).unwrap();
            let ticket = format!("GYRO-{}", encode_ticket(addr, &token));
            let code = normalize_room_code(&ticket).unwrap();
            assert!(normalize_room_code("GYRO-8492").is_err());
            assert!(ticket_qr_svg(&ticket).is_some());

            let (addr, token) = parse_ticket(&code).unwrap();
            let guest = Endpoint::builder(presets::Minimal)
                .relay_mode(iroh::RelayMode::Disabled)
                .bind()
                .await?;
            let wrong_connection = guest.connect(addr.clone(), ALPN).await?;
            let (mut send, mut recv) = wrong_connection.open_bi().await?;
            let mut wrong_token = token;
            wrong_token[0] ^= 1;
            send.write_all(&wrong_token).await?;
            assert!(read_frame::<RemoteSyncPayload>(&mut recv).await.is_err());
            assert!(requests.is_empty());

            let connection = guest.connect(addr, ALPN).await?;
            let (mut send, mut recv) = connection.open_bi().await?;
            send.write_all(&token).await?;
            let mut snapshot = RemoteSyncPayload {
                track: None,
                position_secs: 21.0,
                is_playing: false,
                sent_at_ms: now_ms(),
                allow_controls: true,
                source_name: "Test".into(),
                host_name: "Test".into(),
            };
            host.publish(snapshot.clone());
            let received: RemoteSyncPayload = read_frame(&mut recv).await?;
            assert_eq!(received.position_secs, 21.0);
            assert!(received.allow_controls);

            snapshot.position_secs = 35.0;
            host.publish(snapshot);
            let received: RemoteSyncPayload = read_frame(&mut recv).await?;
            assert_eq!(received.position_secs, 35.0);

            write_frame(&mut send, &RemoteGuestEvent::Control("next".into())).await?;
            let request = requests.recv().await?;
            assert!(matches!(request.command, JamCommand::NextTrack));
            assert!(request.source.is_current(None, Some(&host.invitation)));

            drop(host);
            connection.closed().await;
            guest.close().await;
            Ok::<_, anyhow::Error>(())
        })
        .await?
    }
}
