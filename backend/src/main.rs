/// HTTP endpoints
mod api;
/// Our error enum
mod error;
/// The Order type and related functionality
mod order;
///
mod storage;

pub use error::Result;
use storage::OrderStorage;

#[tokio::main]
async fn main() -> crate::Result<()> {
    // Start tracing
    tracing_subscriber::fmt().compact().init();
    let storage = OrderStorage::new();

    // Start our HTTP API
    let server = api::spawn_server(storage.clone())?;

    // Wait for the HTTP server to close
    server.await?;

    Ok(())
}
