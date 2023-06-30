/// HTTP endpoints
mod api;
/// Our error enum
mod error;
/// The Order type and related functionality
mod order;
/// Abstracts over storage for orders
mod repository;

use std::sync::Arc;

pub use error::Result;
use repository::in_memory::InMemoryStorage;
pub use repository::SharedOrderRepositoryImpl;

#[tokio::main]
async fn main() -> crate::Result<()> {
    // Start tracing

    // tracing_subscriber::fmt().compact().init();

    // If Redis is to be used, simply replace `InMemoryStorage` here
    // for `RedisClient::connect`.
    let storage = Arc::new(InMemoryStorage::new());

    // Start our HTTP API
    let server = api::spawn_server(storage)?;

    // Wait for the HTTP server to close
    server.await?;

    Ok(())
}
