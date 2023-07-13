# Quinn Rustls Compatibility Issue

This is a minimal example to showcase an incompatibility issue when using
rustls `ClientConfig` (or `ServerConfig`) to configure quinn.

Error message:
```
error[E0277]: the trait bound `rustls::ClientConfig: quinn::crypto::ClientConfig` is not satisfied
  --> src/main.rs:10:34
   |
10 |         quinn::ClientConfig::new(Arc::new(rustls_client_config));
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `quinn::crypto::ClientConfig` is not implemented for `rustls::ClientConfig`
   |
   = help: the trait `quinn::crypto::ClientConfig` is implemented for `rustls::client::client_conn::ClientConfig`
   = note: required for the cast from `rustls::ClientConfig` to the object type `dyn quinn::crypto::ClientConfig`

For more information about this error, try `rustc --explain E0277`.
```

Please note that rustls and quinn versions have been pinned, but the issue can
arrise from Cargo's semantic versioning rules on choosing versions. In fact,
this happened to one of our projects.
