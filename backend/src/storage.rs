use std::sync::Arc;

use rustc_hash::FxHashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::order::Order;

/// Safely send T accross threads and allows for safe shared internal
/// mutability using a readers-writers lock.
pub type Shared<T> = Arc<RwLock<T>>;

#[derive(Debug, Clone)]
pub struct OrderStorage {
    /// FxHash from `rustc-hash` has great performance characteristics for
    /// small integer types. Since we'll be hashing UUIDs, which are
    /// sequences of 128 bits, this should result in very fast
    /// lookups.
    inner: Shared<FxHashMap<Uuid, Order>>,
}

impl OrderStorage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(rustc_hash::FxHashMap::default())),
        }
    }

    pub async fn insert(&self, order: Order) {
        // We'll wait until no one else is reading from the storage
        // in order to be able to write into it.

        if let Some(collided_order) =
            self.inner.write().await.insert(order.id, order)
        {
            // If we're here, we inserted a new order into a previous
            // entry, meaning there was an UUID collision.
            //
            // The chances of this occurring are extremely low (one in
            // 2^122).
            tracing::error!(
                "There was an UUID collision for {}",
                collided_order.id
            );
        }
    }

    /// List all orders inserted so far
    pub async fn list_all(&self) -> Vec<Order> {
        self.inner.read().await.values().copied().collect()
    }
}
