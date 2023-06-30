use std::sync::Arc;

use uuid::Uuid;

pub mod in_memory;
pub mod redis;

use crate::{order::Order, Result};

pub type SharedOrderRepositoryImpl =
    Arc<dyn OrderRepository + Sync + Send>;

#[async_trait::async_trait]
pub trait OrderRepository {
    /// Insert a new order into the repository
    async fn insert(&self, order: Order) -> Result;

    /// List all currently inserted orders
    async fn list_all(&self) -> Result<Vec<Order>>;

    /// Remove an order given its UUID
    async fn remove(&self, id_to_remove: &Uuid) -> Result;
}
