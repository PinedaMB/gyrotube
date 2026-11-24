#[cfg(desktop)]
mod desktop {
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use tauri::{AppHandle, Manager};

    const WAKE_MSG: &[u8] = b"GYROTUBE_SHOW\n";

    #[cfg(unix)]
    use std::os::unix::net::{UnixListener, UnixStream};

    #[cfg(unix)]
    fn socket_path() -> PathBuf {
        if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR")
            && !runtime_dir.trim().is_empty()
        {
            return PathBuf::from(runtime_dir).join("gyrotube-single-instance.sock");
        }
        std::env::temp_dir().join("gyrotube-single-instance.sock")
    }

    #[cfg(windows)]
    fn port_file_path() -> PathBuf {
        std::env::temp_dir().join("gyrotube-single-instance.port")
    }

    pub struct SingleInstanceGuard {
        #[cfg(unix)]
        path: PathBuf,
        #[cfg(unix)]
        listener: Option<UnixListener>,
        #[cfg(windows)]
        path: PathBuf,
        #[cfg(windows)]
        listener: Option<std::net::TcpListener>,
    }

    impl SingleInstanceGuard {
        /// Tries to become the primary GyroTube instance.
        /// Returns `None` if an existing instance was detected and notified to show its window.
        pub fn acquire_or_notify_existing() -> Option<Self> {
            #[cfg(unix)]
            {
                let path = socket_path();
                if let Ok(mut stream) = UnixStream::connect(&path) {
                    let _ = stream.write_all(WAKE_MSG);
                    let _ = stream.flush();
                    return None;
                }
                // Remove stale socket left behind by an unclean exit
                let _ = std::fs::remove_file(&path);
                let listener = match UnixListener::bind(&path) {
                    Ok(l) => Some(l),
                    Err(_) => {
                        // Another instance may have won the race; try notifying it once more
                        if let Ok(mut stream) = UnixStream::connect(&path) {
                            let _ = stream.write_all(WAKE_MSG);
                            let _ = stream.flush();
                            return None;
                        }
                        None
                    }
                };
                Some(Self { path, listener })
            }

            #[cfg(windows)]
            {
                let path = port_file_path();
                if let Ok(port_str) = std::fs::read_to_string(&path)
                    && let Ok(port) = port_str.trim().parse::<u16>()
                    && let Ok(mut stream) = std::net::TcpStream::connect(("127.0.0.1", port))
                {
                    let _ = stream.write_all(WAKE_MSG);
                    let _ = stream.flush();
                    return None;
                }
                let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).ok();
                if let Some(ref l) = listener
                    && let Ok(addr) = l.local_addr()
                {
                    let _ = std::fs::write(&path, addr.port().to_string());
                }
                Some(Self { path, listener })
            }
        }

        pub fn start_listener(&mut self, app: AppHandle) {
            let Some(listener) = self.listener.take() else {
                return;
            };
            std::thread::spawn(move || {
                for incoming in listener.incoming() {
                    let Ok(mut stream) = incoming else {
                        continue;
                    };
                    let mut buf = [0u8; 32];
                    let _ = stream.set_read_timeout(Some(std::time::Duration::from_millis(500)));
                    if let Ok(n) = stream.read(&mut buf)
                        && buf[..n].starts_with(b"GYROTUBE_SHOW")
                        && let Some(win) = app.get_webview_window("main")
                    {
                        let _ = win.show();
                        let _ = win.unminimize();
                        let _ = win.set_focus();
                    }
                }
            });
        }
    }

    impl Drop for SingleInstanceGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

#[cfg(desktop)]
pub use desktop::SingleInstanceGuard;
