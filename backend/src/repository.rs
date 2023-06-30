use uuid::Uuid;

pub mod in_memory;

use crate::{order::Order, Result};

#[async_trait::async_trait]
pub trait OrderRepository {
    /// Insert a new order into the repository
    async fn insert(&self, order: Order) -> Result;

    /// List all currently inserted orders
    async fn list_all(&self) -> Result<Vec<Order>>;

    /// Remove an order given its UUID
    async fn remove(&self, id_to_remove: &Uuid) -> Result<Order>;
}
