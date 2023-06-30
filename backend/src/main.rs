/// HTTP endpoints
mod api;
/// Our error enum
mod error;
/// The Order type and related functionality
mod order;
/// Abstracts over storage for orders
mod repository;

pub use error::Result;
use repository::{in_memory::InMemoryStorage, redis::RedisClient};

#[tokio::main]
async fn main() -> crate::Result<()> {
    // Start tracing

    // tracing_subscriber::fmt().compact().init();
    // let storage = InMemoryStorage::new();
    let storage = RedisClient::connect("redis://127.0.0.1:6379")?;

    // Start our HTTP API
    let server = api::spawn_server(storage)?;

    // Wait for the HTTP server to close
    server.await?;

    Ok(())
}
