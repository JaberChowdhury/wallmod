//! Global Tokio background runtime integration for GPUI tasks.

use std::sync::OnceLock;

static TOKIO_RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

/// Returns a reference to the global static Tokio runtime.
pub fn get_runtime() -> &'static tokio::runtime::Runtime {
    TOKIO_RT.get_or_init(|| {
        tokio::runtime::Runtime::new().expect("Failed to initialize global Tokio runtime")
    })
}

/// Runs a blocking function on the global Tokio thread pool.
pub fn spawn_blocking<F, R>(f: F) -> tokio::task::JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    get_runtime().spawn_blocking(f)
}
