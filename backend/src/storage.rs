use std::{hash::BuildHasherDefault, sync::Arc};

use dashmap::DashMap;
use rustc_hash::FxHasher;
use uuid::Uuid;

use crate::order::Order;

#[derive(Debug, Clone)]
pub struct OrderStorage {
    /// FxHash from `rustc-hash` has great performance characteristics for
    /// small integer types. Since we'll be hashing UUIDs, which are
    /// sequences of 128 bits, this should result in very fast
    /// lookups.
    inner: Arc<DashMap<Uuid, Order, BuildHasherDefault<FxHasher>>>,
}

impl OrderStorage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(
                DashMap::with_capacity_and_hasher_and_shard_amount(
                    5_000,
                    Default::default(),
                    256,
                ),
            ),
        }
    }

    pub fn insert(&self, order: Order) {
        // We'll wait until no one else is reading from the storage
        // in order to be able to write into it.

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
    }

    /// List all orders inserted so far
    pub async fn list_all(&self) -> Vec<Order> {
        let orders: Vec<_> =
            self.inner.iter().map(|entry| *entry.value()).collect();

        tracing::info!("Current order count: {}", orders.len());

        orders
    }
}
