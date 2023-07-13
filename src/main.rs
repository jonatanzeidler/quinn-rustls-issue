use std::sync::Arc;

fn main() {
    let rustls_client_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(rustls::RootCertStore::empty())
        .with_no_client_auth();

    let _quinn_client_config =
        quinn::ClientConfig::new(Arc::new(rustls_client_config));
    // ...
}
