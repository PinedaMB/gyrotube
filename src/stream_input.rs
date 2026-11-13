use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use gtk4::gio;
use gtk4::glib;
use gtk4::glib::subclass::prelude::*;
use gtk4::gio::prelude::*;
use gtk4::gio::subclass::prelude::*;

use crate::backend::runtime;

const PAGE_SIZE: u64 = 256 * 1024;

#[derive(Clone)]
pub struct StreamSource {
    inner: Arc<StreamSourceInner>,
}

impl std::fmt::Debug for StreamSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamSource")
            .field("total_size", &self.inner.total_size)
            .finish()
    }
}

struct StreamSourceInner {
    url: String,
    user_agent: String,
    http: reqwest::Client,
    total_size: u64,
    pages: Mutex<HashMap<usize, Arc<Vec<u8>>>>,
    fetching: Mutex<HashSet<usize>>,
    cache_path: Option<std::path::PathBuf>,
    saved_to_disk: Mutex<bool>,
}

impl StreamSource {
    pub async fn open(
        http: reqwest::Client,
        url: String,
        user_agent: String,
        cache_path: Option<std::path::PathBuf>,
    ) -> Result<Self, reqwest::Error> {
        let resp = http
            .get(&url)
            .header(reqwest::header::USER_AGENT, &user_agent)
            .header(reqwest::header::RANGE, format!("bytes=0-{}", PAGE_SIZE - 1))
            .send()
            .await?
            .error_for_status()?;

        let status = resp.status();
        let content_range_total = resp
            .headers()
            .get(reqwest::header::CONTENT_RANGE)
            .and_then(|v| v.to_str().ok())
            .and_then(|cr| cr.rsplit('/').next())
            .and_then(|t| t.parse::<u64>().ok());

        let first_bytes = resp.bytes().await?.to_vec();
        let mut pages = HashMap::new();

        let total_size = if status == reqwest::StatusCode::PARTIAL_CONTENT
            && let Some(total) = content_range_total
            && total > first_bytes.len() as u64
        {
            pages.insert(0, Arc::new(first_bytes));
            total
        } else {
            let total = first_bytes.len() as u64;
            for (idx, chunk) in first_bytes.chunks(PAGE_SIZE as usize).enumerate() {
                pages.insert(idx, Arc::new(chunk.to_vec()));
            }
            total
        };

        let source = Self {
            inner: Arc::new(StreamSourceInner {
                url,
                user_agent,
                http,
                total_size,
                pages: Mutex::new(pages),
                fetching: Mutex::new(HashSet::new()),
                cache_path,
                saved_to_disk: Mutex::new(false),
            }),
        };

        // Buffer remaining pages in background so seeking anywhere is instant
        source.prefetch_all_remaining();
        Ok(source)
    }

    fn prefetch_all_remaining(&self) {
        let num_pages = self.num_pages();
        if num_pages <= 1 {
            return;
        }
        let source = self.clone();
        runtime().spawn(async move {
            let sem = Arc::new(tokio::sync::Semaphore::new(2));
            let mut join_set = tokio::task::JoinSet::new();
            for page_idx in 1..num_pages {
                let s = source.clone();
                let sem = Arc::clone(&sem);
                join_set.spawn(async move {
                    let _permit = sem.acquire().await.ok();
                    let _ = s.fetch_page_async(page_idx).await;
                });
            }
            while join_set.join_next().await.is_some() {}
        });
    }

    pub fn into_gio_stream(self) -> gio::InputStream {
        HttpRangeInputStream::new(self).upcast()
    }

    fn num_pages(&self) -> usize {
        if self.inner.total_size == 0 {
            0
        } else {
            self.inner.total_size.div_ceil(PAGE_SIZE) as usize
        }
    }

    fn prefetch_page(&self, page_idx: usize) {
        if page_idx >= self.num_pages() {
            return;
        }
        {
            let Ok(pages) = self.inner.pages.lock() else {
                return;
            };
            if pages.contains_key(&page_idx) {
                return;
            }
        }
        {
            let Ok(mut fetching) = self.inner.fetching.lock() else {
                return;
            };
            if !fetching.insert(page_idx) {
                return;
            }
        }

        let source = self.clone();
        runtime().spawn(async move {
            let _ = source.fetch_page_async(page_idx).await;
            if let Ok(mut fetching) = source.inner.fetching.lock() {
                fetching.remove(&page_idx);
            }
        });
    }

    async fn fetch_page_async(&self, page_idx: usize) -> Result<Arc<Vec<u8>>, String> {
        if let Ok(pages) = self.inner.pages.lock()
            && let Some(page) = pages.get(&page_idx)
        {
            return Ok(Arc::clone(page));
        }

        let start = page_idx as u64 * PAGE_SIZE;
        let end = (start + PAGE_SIZE - 1).min(self.inner.total_size.saturating_sub(1));
        let resp = self
            .inner
            .http
            .get(&self.inner.url)
            .header(reqwest::header::USER_AGENT, &self.inner.user_agent)
            .header(reqwest::header::RANGE, format!("bytes={start}-{end}"))
            .send()
            .await
            .map_err(|e| format!("Error de red en stream: {}", e.without_url()))?
            .error_for_status()
            .map_err(|e| format!("HTTP {} en stream", e.status().map(|s| s.as_u16()).unwrap_or(0)))?;

        let bytes = Arc::new(
            resp.bytes()
                .await
                .map_err(|e| format!("Error leyendo chunk: {e}"))?
                .to_vec(),
        );

        let complete = {
            let Ok(mut pages) = self.inner.pages.lock() else {
                return Ok(bytes);
            };
            pages.insert(page_idx, Arc::clone(&bytes));
            pages.len() == self.num_pages()
        };

        if complete {
            self.maybe_flush_to_disk();
        }

        Ok(bytes)
    }

    fn maybe_flush_to_disk(&self) {
        let Some(ref path) = self.inner.cache_path else {
            return;
        };
        if path.is_file() {
            return;
        }
        {
            let Ok(mut saved) = self.inner.saved_to_disk.lock() else {
                return;
            };
            if *saved {
                return;
            }
            *saved = true;
        }

        let source = self.clone();
        let path = path.clone();
        runtime().spawn(async move {
            let num_pages = source.num_pages();
            let mut full = Vec::with_capacity(source.inner.total_size as usize);
            {
                let Ok(pages) = source.inner.pages.lock() else {
                    return;
                };
                for idx in 0..num_pages {
                    let Some(p) = pages.get(&idx) else {
                        return;
                    };
                    full.extend_from_slice(p);
                }
            }
            if let Some(parent) = path.parent() {
                let tmp = path.with_extension("part");
                let _ = parent;
                if std::fs::write(&tmp, full).is_ok() {
                    let _ = std::fs::rename(&tmp, &path);
                }
            }
        });
    }

    fn read_at(&self, offset: u64, out: &mut [u8]) -> Result<usize, glib::Error> {
        if offset >= self.inner.total_size || out.is_empty() {
            return Ok(0);
        }

        let page_idx = (offset / PAGE_SIZE) as usize;
        let in_page = (offset % PAGE_SIZE) as usize;

        let page = if let Ok(pages) = self.inner.pages.lock()
            && let Some(p) = pages.get(&page_idx)
        {
            Arc::clone(p)
        } else {
            runtime()
                .block_on(self.fetch_page_async(page_idx))
                .map_err(|msg| glib::Error::new(gio::IOErrorEnum::Failed, &msg))?
        };

        // Keep 1 page ahead buffered in the background
        self.prefetch_page(page_idx + 1);

        if in_page >= page.len() {
            return Ok(0);
        }
        let avail = page.len() - in_page;
        let to_copy = avail.min(out.len());
        out[..to_copy].copy_from_slice(&page[in_page..in_page + to_copy]);
        Ok(to_copy)
    }
}

mod imp {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[derive(Default)]
    pub struct HttpRangeInputStream {
        pub source: Mutex<Option<StreamSource>>,
        pub pos: AtomicU64,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HttpRangeInputStream {
        const NAME: &'static str = "GyroTubeHttpRangeInputStream";
        type Type = super::HttpRangeInputStream;
        type ParentType = gio::InputStream;
        type Interfaces = (gio::Seekable,);
    }

    impl ObjectImpl for HttpRangeInputStream {}

    impl InputStreamImpl for HttpRangeInputStream {
        fn read(
            &self,
            buffer: &mut [u8],
            _cancellable: Option<&gio::Cancellable>,
        ) -> Result<usize, glib::Error> {
            let source = self
                .source
                .lock()
                .ok()
                .and_then(|g| g.clone())
                .ok_or_else(|| glib::Error::new(gio::IOErrorEnum::Closed, "Stream closed"))?;
            let pos = self.pos.load(Ordering::Relaxed);
            let n = source.read_at(pos, buffer)?;
            self.pos.store(pos + n as u64, Ordering::Relaxed);
            Ok(n)
        }

        fn close(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
            if let Ok(mut g) = self.source.lock() {
                *g = None;
            }
            Ok(())
        }
    }

    impl SeekableImpl for HttpRangeInputStream {
        fn tell(&self) -> i64 {
            self.pos.load(Ordering::Relaxed) as i64
        }

        fn can_seek(&self) -> bool {
            true
        }

        fn seek(
            &self,
            offset: i64,
            type_: glib::SeekType,
            _cancellable: Option<&gio::Cancellable>,
        ) -> Result<(), glib::Error> {
            let total = self
                .source
                .lock()
                .ok()
                .and_then(|g| g.as_ref().map(|s| s.inner.total_size))
                .unwrap_or(0);
            let cur = self.pos.load(Ordering::Relaxed) as i64;
            let new_pos = match type_ {
                glib::SeekType::Set => offset,
                glib::SeekType::Cur => cur + offset,
                glib::SeekType::End => total as i64 + offset,
                _ => {
                    return Err(glib::Error::new(
                        gio::IOErrorEnum::NotSupported,
                        "Unsupported seek type",
                    ));
                }
            };
            if new_pos < 0 || new_pos as u64 > total {
                return Err(glib::Error::new(
                    gio::IOErrorEnum::InvalidArgument,
                    "Seek out of bounds",
                ));
            }
            self.pos.store(new_pos as u64, Ordering::Relaxed);
            Ok(())
        }

        fn can_truncate(&self) -> bool {
            false
        }

        fn truncate(
            &self,
            _offset: i64,
            _cancellable: Option<&gio::Cancellable>,
        ) -> Result<(), glib::Error> {
            Err(glib::Error::new(
                gio::IOErrorEnum::NotSupported,
                "Truncate not supported",
            ))
        }
    }
}

glib::wrapper! {
    pub struct HttpRangeInputStream(ObjectSubclass<imp::HttpRangeInputStream>)
        @extends gio::InputStream,
        @implements gio::Seekable;
}

impl HttpRangeInputStream {
    pub fn new(source: StreamSource) -> Self {
        let obj: Self = glib::Object::new();
        if let Ok(mut g) = obj.imp().source.lock() {
            *g = Some(source);
        }
        obj
    }
}
