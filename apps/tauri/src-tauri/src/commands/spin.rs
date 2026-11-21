use std::sync::Arc;

use gyrotube_core::jam::{JamTrackInfo, generate_qr_svg, start_jam_server};
use gyrotube_core::models::Track;
use gyrotube_core::spin_remote::{
    HostPublisher, RemoteRole, RemoteSyncPayload, local_display_name, normalize_room_code, now_ms,
    start_guest_sync, ticket_qr_svg,
};
use serde::Serialize;
use tauri::State;

use super::{AppCoreState, SpinState};

#[derive(Serialize)]
pub struct SpinStatusPayload {
    pub local_active: bool,
    pub local_url: String,
    pub local_qr_svg: Option<String>,
    pub allow_controls: bool,
    pub guest_count: usize,
    pub added_count: usize,
    pub remote_role: &'static str,
    pub remote_ticket: Option<String>,
    pub remote_qr_svg: Option<String>,
}

fn build_spin_payload(spin: &SpinState) -> SpinStatusPayload {
    let snap = spin.snapshot.read().ok();
    let local_active = spin.server.is_some();
    let local_url = snap.as_ref().map(|n| n.url.clone()).unwrap_or_default();
    let local_qr_svg = if local_active && !local_url.is_empty() {
        generate_qr_svg(&local_url)
    } else {
        None
    };
    let (remote_role, remote_ticket, remote_qr_svg) = match &spin.remote_role {
        RemoteRole::Idle => ("idle", None, None),
        RemoteRole::Host(code) => {
            let ticket = format!("GYRO-{code}");
            let qr = ticket_qr_svg(&ticket);
            ("host", Some(ticket), qr)
        }
        RemoteRole::Guest(target) => ("guest", Some(target.clone()), None),
    };
    SpinStatusPayload {
        local_active,
        local_url,
        local_qr_svg,
        allow_controls: snap.as_ref().map(|n| n.allow_controls).unwrap_or(false),
        guest_count: snap.as_ref().map(|n| n.guest_count).unwrap_or(0),
        added_count: snap.as_ref().map(|n| n.added_count).unwrap_or(0),
        remote_role,
        remote_ticket,
        remote_qr_svg,
    }
}

#[tauri::command]
pub async fn spin_status(state: State<'_, AppCoreState>) -> Result<SpinStatusPayload, String> {
    let spin = state.spin.lock().map_err(|e| e.to_string())?;
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub async fn spin_start_local(state: State<'_, AppCoreState>) -> Result<SpinStatusPayload, String> {
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    if spin.server.is_none() {
        let handle = start_jam_server(
            state.backend.clone(),
            Arc::clone(&spin.snapshot),
            spin.cmd_tx.clone(),
        )?;
        let url = handle.url.clone();
        if let Ok(mut sn) = spin.snapshot.write() {
            sn.active = true;
            sn.url = url;
            sn.host_name = local_display_name();
            sn.guests.clear();
            sn.guest_list.clear();
            sn.guest_count = 0;
            sn.added_count = 0;
        }
        spin.server = Some(handle);
    }
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_stop_local(state: State<'_, AppCoreState>) -> Result<SpinStatusPayload, String> {
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    if let Some(mut srv) = spin.server.take() {
        srv.stop();
    }
    if let Ok(mut sn) = spin.snapshot.write() {
        sn.active = false;
    }
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_set_allow_controls(
    state: State<'_, AppCoreState>,
    allow: bool,
) -> Result<SpinStatusPayload, String> {
    let spin = state.spin.lock().map_err(|e| e.to_string())?;
    if let Ok(mut sn) = spin.snapshot.write() {
        sn.allow_controls = allow;
    }
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub async fn spin_start_remote_host(
    state: State<'_, AppCoreState>,
) -> Result<SpinStatusPayload, String> {
    let cmd_tx = {
        let spin = state.spin.lock().map_err(|e| e.to_string())?;
        spin.cmd_tx.clone()
    };
    let publisher = HostPublisher::new(cmd_tx)
        .await
        .map_err(|e| e.to_string())?;
    let code = publisher.invitation.clone();
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    spin.guest_session = None;
    spin.host_publisher = Some(publisher);
    spin.remote_role = RemoteRole::Host(code);
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_stop_remote(state: State<'_, AppCoreState>) -> Result<SpinStatusPayload, String> {
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    spin.host_publisher = None;
    spin.guest_session = None;
    spin.remote_role = RemoteRole::Idle;
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub fn spin_join_remote(
    state: State<'_, AppCoreState>,
    code: String,
) -> Result<SpinStatusPayload, String> {
    let target = normalize_room_code(&code)?;
    let mut spin = state.spin.lock().map_err(|e| e.to_string())?;
    spin.host_publisher = None;
    spin.remote_role = RemoteRole::Guest(target.clone());
    let tx = spin.remote_tx.clone();
    spin.guest_session = Some(start_guest_sync(target, tx));
    Ok(build_spin_payload(&spin))
}

#[tauri::command]
pub async fn spin_sync_snapshot(
    state: State<'_, AppCoreState>,
    is_playing: bool,
    position_secs: f64,
    source_name: String,
    current_index: Option<usize>,
    queue: Vec<Track>,
) -> Result<(), String> {
    let spin = state.spin.lock().map_err(|e| e.to_string())?;
    let current_track = current_index.and_then(|idx| queue.get(idx)).cloned();
    let jam_queue: Vec<JamTrackInfo> = queue
        .iter()
        .enumerate()
        .map(|(i, t)| JamTrackInfo {
            id: t.id.clone(),
            title: t.title.clone(),
            artist: t.artist.clone(),
            album: t.album.clone(),
            duration_secs: t.duration_secs,
            cover_url: t.cover_url.clone(),
            is_current: current_index == Some(i),
        })
        .collect();
    let now_playing = current_index.and_then(|idx| jam_queue.get(idx).cloned());

    let allow_controls = if let Ok(mut sn) = spin.snapshot.write() {
        sn.is_playing = is_playing;
        sn.position_secs = position_secs;
        sn.now_playing = now_playing;
        sn.queue = jam_queue;
        sn.allow_controls
    } else {
        false
    };

    if let Some(publisher) = &spin.host_publisher {
        publisher.publish(RemoteSyncPayload {
            track: current_track,
            position_secs,
            is_playing,
            sent_at_ms: now_ms(),
            allow_controls,
            source_name,
            host_name: local_display_name(),
        });
    }
    Ok(())
}
