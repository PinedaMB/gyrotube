use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use symphonia::core::io::MediaSource;
use tokio::sync::watch;

use crate::audio_streamer::ChunkProgress;

const WAIT_STEP: Duration = Duration::from_millis(8);
const MAX_WAIT_FOR_DATA: Duration = Duration::from_secs(15);

/// `symphonia::core::io::MediaSource` that reads directly from a progressively downloaded
/// disk file managed by `AudioStreamer`, waiting cleanly when the decoder catches up to
/// the writer without needing loopback HTTP sockets.
pub struct ProgressiveFileReader {
    file: File,
    pos: u64,
    static_len: Option<u64>,
    progress_rx: Option<watch::Receiver<ChunkProgress>>,
    cancelled: Arc<AtomicBool>,
}

impl ProgressiveFileReader {
    pub fn open(
        path: &Path,
        progress_rx: Option<watch::Receiver<ChunkProgress>>,
        cancelled: Arc<AtomicBool>,
    ) -> io::Result<Self> {
        let file = File::open(path)?;
        let static_len = if progress_rx.is_none() {
            file.metadata().ok().map(|m| m.len())
        } else {
            None
        };
        Ok(Self {
            file,
            pos: 0,
            static_len,
            progress_rx,
            cancelled,
        })
    }

    fn wait_for_offset(&self, needed_end: u64) -> io::Result<bool> {
        let Some(ref rx) = self.progress_rx else {
            return Ok(true);
        };
        let deadline = Instant::now() + MAX_WAIT_FOR_DATA;
        loop {
            if self.cancelled.load(Ordering::Relaxed) {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "decoder cancelled"));
            }
            let snap = rx.borrow().clone();
            if let Some(err) = snap.error {
                return Err(io::Error::other(err));
            }
            if snap.written >= needed_end {
                return Ok(true);
            }
            if snap.done {
                return Ok(snap.written > self.pos);
            }
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "timed out waiting for audio chunk",
                ));
            }
            std::thread::sleep(WAIT_STEP);
        }
    }
}

impl Read for ProgressiveFileReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        if self.cancelled.load(Ordering::Relaxed) {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "decoder cancelled"));
        }

        // Wait until at least 1 byte is available at `self.pos`
        if !self.wait_for_offset(self.pos.saturating_add(1))? {
            return Ok(0);
        }

        let max_readable = if let Some(ref rx) = self.progress_rx {
            let written = rx.borrow().written;
            written.saturating_sub(self.pos) as usize
        } else {
            buf.len()
        };

        let to_read = buf.len().min(max_readable.max(1));
        self.file.seek(SeekFrom::Start(self.pos))?;
        let n = self.file.read(&mut buf[..to_read])?;
        self.pos = self.pos.saturating_add(n as u64);
        Ok(n)
    }
}

impl Seek for ProgressiveFileReader {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let len = self.byte_len().unwrap_or(0) as i64;
        let next = match pos {
            SeekFrom::Start(off) => off as i64,
            SeekFrom::Current(delta) => (self.pos as i64).saturating_add(delta),
            SeekFrom::End(delta) => len.saturating_add(delta),
        };
        if next < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cannot seek before start of stream",
            ));
        }
        self.pos = next as u64;
        self.file.seek(SeekFrom::Start(self.pos))
    }
}

impl MediaSource for ProgressiveFileReader {
    fn is_seekable(&self) -> bool {
        true
    }

    fn byte_len(&self) -> Option<u64> {
        if let Some(ref rx) = self.progress_rx {
            let snap = rx.borrow();
            snap.total_bytes.or_else(|| snap.done.then_some(snap.written))
        } else {
            self.static_len
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::time::Duration;

    use tokio::sync::watch;

    use super::ProgressiveFileReader;
    use crate::audio_streamer::ChunkProgress;

    #[test]
    fn reads_and_seeks_across_progressively_written_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("stream.part.m4a");
        std::fs::write(&path, b"hello ").unwrap();

        let (tx, rx) = watch::channel(ChunkProgress {
            written: 6,
            total_bytes: Some(11),
            done: false,
            error: None,
        });

        let writer_path = path.clone();
        let writer = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(30));
            let mut f = std::fs::OpenOptions::new()
                .append(true)
                .open(&writer_path)
                .unwrap();
            f.write_all(b"world").unwrap();
            f.flush().unwrap();
            let _ = tx.send(ChunkProgress {
                written: 11,
                total_bytes: Some(11),
                done: true,
                error: None,
            });
        });

        let mut reader =
            ProgressiveFileReader::open(&path, Some(rx), Arc::new(AtomicBool::new(false))).unwrap();
        let mut all = Vec::new();
        reader.read_to_end(&mut all).unwrap();
        assert_eq!(all, b"hello world");

        reader.seek(SeekFrom::Start(6)).unwrap();
        let mut tail = [0u8; 5];
        reader.read_exact(&mut tail).unwrap();
        assert_eq!(&tail, b"world");

        writer.join().unwrap();
    }
}
