use std::{hash::BuildHasherDefault, sync::Arc};

use dashmap::DashMap;
use rustc_hash::FxHasher;
use uuid::Uuid;

use super::OrderRepository;
use crate::{error::Error, order::Order, Result};

#[derive(Debug, Clone)]
pub struct InMemoryStorage {
    /// FxHash from `rustc-hash` has great performance characteristics for
    /// small integer types. Since we'll be hashing UUIDs, which are
    /// sequences of 128 bits, this should result in very fast
    /// lookups.
    ///
    /// Other than that, we use `DashMap` since it uses many shards each
    /// with their own RwLock, which reduces lock contention.
    ///
    /// Before `DashMap` I was using `tokio::sync::RwLock<HashMap<K, V>>`,
    /// but since that only utilizes one lock for the entire map, lock
    /// contention was high.
    ///
    /// The con of `DashMap` (in my opinion) is that it does not use an
    /// async RwLock.
    inner: Arc<DashMap<Uuid, Order, BuildHasherDefault<FxHasher>>>,
}

#[async_trait::async_trait]
impl OrderRepository for InMemoryStorage {
    /// Insert a new order into the repository
    async fn insert(&self, order: Order) -> Result {
        tracing::info!("Inserting order into storage");

        if let Some(collided_order) = self.inner.insert(order.id, order) {
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

        Ok(())
    }

    /// List all currently inserted orders
    async fn list_all(&self) -> Result<Vec<Order>> {
        let orders: Vec<_> =
            self.inner.iter().map(|entry| *entry.value()).collect();

        tracing::info!("Current order count: {}", orders.len());

        Ok(orders)
    }

    /// Remove an order given its UUID
    async fn remove(&self, id_to_remove: &Uuid) -> Result<()> {
        self.inner
            .remove(&id_to_remove)
            .map(|(_id, _order)| ())
            .ok_or(Error::NotFound)
    }
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(
                DashMap::with_capacity_and_hasher_and_shard_amount(
                    5_000,
                    Default::default(),
                    // Start with plenty of shards to avoid reallocation
                    // and to greatly reduce
                    // lock contention
                    256,
                ),
            ),
        }
    }
}
