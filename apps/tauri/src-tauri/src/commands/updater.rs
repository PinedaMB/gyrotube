use gyrotube_core::updater::{UpdateCheckResult, check_github_release};
use tauri::AppHandle;
#[cfg(desktop)]
use tauri::Emitter;

#[cfg(desktop)]
#[derive(Clone, serde::Serialize)]
pub struct UpdateProgressEvent {
    pub downloaded: u64,
    pub total: Option<u64>,
}

#[cfg(desktop)]
fn can_use_native_updater() -> bool {
    #[cfg(target_os = "linux")]
    {
        std::env::var_os("APPIMAGE").is_some()
    }
    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> Result<UpdateCheckResult, String> {
    let current = env!("CARGO_PKG_VERSION");

    #[cfg(desktop)]
    if can_use_native_updater() {
        use tauri_plugin_updater::UpdaterExt;
        if let Ok(updater) = app.updater()
            && let Ok(Some(update)) = updater.check().await
        {
            return Ok(UpdateCheckResult {
                current_version: current.to_string(),
                latest_version: Some(update.version.clone()),
                update_available: true,
                can_auto_install: true,
                release_url: format!(
                    "https://github.com/PinedaMB/gyrotube/releases/tag/v{}",
                    update.version.trim_start_matches(['v', 'V'])
                ),
                download_url: None,
                notes: update.body.clone(),
            });
        }
    }

    #[cfg(not(desktop))]
    let _ = app;

    check_github_release(current).await
}

#[tauri::command]
pub async fn install_update(app: AppHandle) -> Result<(), String> {
    #[cfg(desktop)]
    {
        use std::sync::atomic::{AtomicU64, Ordering};
        use tauri_plugin_updater::UpdaterExt;

        let updater = app
            .updater()
            .map_err(|e| format!("Updater not configured: {e}"))?;
        let Some(update) = updater
            .check()
            .await
            .map_err(|e| format!("Failed to check for updates: {e}"))?
        else {
            return Err("No update available".to_string());
        };

        let downloaded = AtomicU64::new(0);
        let handle = app.clone();
        update
            .download_and_install(
                move |chunk_len, content_len| {
                    let prev = downloaded.fetch_add(chunk_len as u64, Ordering::Relaxed);
                    let _ = handle.emit(
                        "updater:progress",
                        UpdateProgressEvent {
                            downloaded: prev + chunk_len as u64,
                            total: content_len,
                        },
                    );
                },
                || {},
            )
            .await
            .map_err(|e| format!("Failed to install update: {e}"))?;

        app.restart();
    }

    #[cfg(not(desktop))]
    {
        let _ = app;
        Err("Auto-install is not supported on this platform; open the download link instead."
            .to_string())
    }
}
