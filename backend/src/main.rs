/// HTTP endpoints
mod api;
/// Our error enum
mod error;
/// The Order type and related functionality
mod order;
/// Abstracts over storage for orders
mod repository;

use std::{borrow::Cow, sync::Arc};

pub use error::Result;
use repository::in_memory::InMemoryStorage;
pub use repository::SharedOrderRepositoryImpl;

#[tokio::main]
async fn main() -> crate::Result<()> {
    // Start tracing
    tracing_subscriber::fmt().compact().init();

    // If Redis is to be used, simply replace `InMemoryStorage` here
    // for `RedisClient::connect`.
    let storage = Arc::new(InMemoryStorage::new());

    // Start our HTTP API
    let server = api::spawn_server(storage, address_from_env())?;

    // Wait for the HTTP server to close
    server.await?;

    Ok(())
}

fn address_from_env() -> (Cow<'static, str>, u16) {
    let address: Cow<'_, str> = std::env::var("ADDRESS")
        .map(Into::into)
        .unwrap_or("127.0.0.1".into());

    let port: u16 = std::env::var("PORT")
        .map(|port| port.parse())
        .into_iter()
        .flatten()
        .next()
        .unwrap_or(8080);

    (address, port)
}
