use std::sync::Arc;

use rustls::{ClientConfig, RootCertStore};

/// Ensures a default rustls CryptoProvider is installed process-wide.
/// Needed when both `ring` and `aws-lc-rs` appear in the dependency tree.
pub fn ensure_crypto_provider() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}

/// Builds a `reqwest::ClientBuilder` using embedded `webpki-roots` certificates.
/// This bypasses `rustls-platform-verifier`'s Android JNI initialization requirement
/// so HTTPS requests work out of the box on Android as well as Linux/Windows/macOS.
pub fn http_client_builder() -> reqwest::ClientBuilder {
    ensure_crypto_provider();
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let mut tls_config = ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("valid TLS protocol versions")
    .with_root_certificates(root_store)
    .with_no_client_auth();

    tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    reqwest::Client::builder().use_preconfigured_tls(tls_config)
}
