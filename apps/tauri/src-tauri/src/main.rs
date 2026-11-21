#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "linux")]
fn configure_appimage_gstreamer() {
    if std::env::var_os("APPIMAGE").is_none() {
        return;
    }
    let candidates = [
        "/usr/lib64/gstreamer-1.0",
        "/usr/lib/x86_64-linux-gnu/gstreamer-1.0",
        "/usr/lib/aarch64-linux-gnu/gstreamer-1.0",
        "/usr/lib/gstreamer-1.0",
    ];
    let existing: Vec<&str> = candidates
        .into_iter()
        .filter(|p| std::path::Path::new(p).is_dir())
        .collect();
    if existing.is_empty() {
        return;
    }
    let current = std::env::var("GST_PLUGIN_SYSTEM_PATH_1_0").unwrap_or_default();
    let mut parts: Vec<String> = current
        .split(':')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    for dir in existing {
        if !parts.iter().any(|p| p == dir) {
            parts.push(dir.to_owned());
        }
    }
    let joined = parts.join(":");
    // SAFETY: Called at the very start of main() before any worker threads are spawned.
    unsafe {
        std::env::set_var("GST_PLUGIN_SYSTEM_PATH_1_0", &joined);
    }
}

fn main() {
    #[cfg(target_os = "linux")]
    configure_appimage_gstreamer();
    gyrotube_tauri_lib::run();
}
