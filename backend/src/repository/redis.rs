use std::debug_assert_eq;

use uuid::Uuid;

use super::OrderRepository;
use crate::{order::Order, Result};

pub type NamespacedId = heapless::Vec<u8, 24>;

enum OrderCodec {}

impl OrderCodec {
    /// Encode an [`Order`] into bytes through [`bincode`].
    fn encode(order: &Order) -> Result<[u8; 44]> {
        let mut buf = [0_u8; 44];

        bincode::serialize_into(&mut buf[..], order)?;

        Ok(buf)
    }

    /// Decode an [`Order`] from bytes through [`bincode`].
    fn decode(bytes: &[u8]) -> Result<Order> {
        bincode::deserialize(bytes).map_err(Into::into)
    }
}

#[derive(Clone)]
pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    #[allow(unused)]
    pub fn connect(redis_address: &str) -> Result<Self> {
        let client = redis::Client::open(redis_address)?;

        tracing::info!("Got Redis connection!");

        Ok(Self { client })
    }

    /// Gets the byte representation of an UUID prefixed with "orderid:".
    ///
    /// Since a UUID is always 16 bytes long, we only require 24 bytes in
    /// the stack to represent this.
    fn namespaced_uuid(uuid: &Uuid) -> NamespacedId {
        debug_assert_eq!(
            b"orderid:".len() + std::mem::size_of::<Uuid>(),
            24
        );

        let mut buf = heapless::Vec::new();

        // Safety: this unwrap could never happen
        buf.extend_from_slice(b"orderid:")
            .and_then(|_| buf.extend_from_slice(uuid.as_bytes()))
            .unwrap();

        buf
    }
}

#[async_trait::async_trait]
impl OrderRepository for RedisClient {
    async fn insert(&self, order: Order) -> Result {
        let mut conn = self.client.get_async_connection().await?;
        let namespaced_id = Self::namespaced_uuid(&order.id);
        let encoded_order = OrderCodec::encode(&order)?;

        redis::cmd("SET")
            .arg(namespaced_id.as_slice())
            .arg(&encoded_order[..])
            .query_async(&mut conn)
            .await?;

        Ok(())
    }

    // TODO: Perhaps use repeated calls to SCAN here.
    //
    //       Although Redis warns agains `KEYS`, it's still way too fast
    //       for our purposes and scale.
    async fn list_all(&self) -> Result<Vec<Order>> {
        let mut conn = self.client.get_async_connection().await?;

        // Get all keys that match the pattern
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg("orderid:*")
            .query_async(&mut conn)
            .await?;

        let mget = {
            let mut mget = redis::cmd("MGET");

            for key in keys {
                mget.arg(key);
            }

            mget
        };

        let orders: Vec<Vec<u8>> = mget.query_async(&mut conn).await?;

        orders
            .into_iter()
            .map(|encoded| OrderCodec::decode(&encoded))
            .collect()
    }

    async fn remove(&self, id_to_remove: &Uuid) -> Result {
        let mut conn = self.client.get_async_connection().await?;
        let namespaced_id = Self::namespaced_uuid(id_to_remove);

        redis::cmd("DEL")
            .arg(namespaced_id.as_slice())
            .query_async(&mut conn)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use uuid::Uuid;

    use crate::{
        order::{Order, OrderSide},
        repository::redis::OrderCodec,
    };

    #[test]
    fn encodes_orders_in_44_bytes() {
        let order = Order {
            id: Uuid::new_v4(),
            quantity: 225_000_220_294,
            price: usize::MAX,
            side: OrderSide::Bid,
        };
        let serialized = bincode::serialize(&order).unwrap();

        assert_eq!(bincode::serialized_size(&order).unwrap(), 44);
        assert_eq!(serialized.len(), 44);
        assert_eq!(
            OrderCodec::encode(&order).unwrap().as_slice(),
            &serialized
        );
    }

    #[test]
    fn encodes_and_decodes_orders_correctly() {
        for idx in 0..5000 {
            let order = Order {
                id: Uuid::new_v4(),
                quantity: fastrand::usize(50..9000) * idx,
                price: fastrand::usize(100..5000) * idx,
                side: if fastrand::bool() {
                    OrderSide::Ask
                } else {
                    OrderSide::Bid
                },
            };

            let encoded = OrderCodec::encode(&order).unwrap();
            let decoded = OrderCodec::decode(&encoded).unwrap();

            assert_eq!(decoded, order);
        }
    }
}
