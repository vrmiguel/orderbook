use serde::{Deserialize, Serialize};

use crate::order::Order;

/// Input form for endpoints to create orders.
///
/// The order side is known from which endpoint was hit.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PartialOrder {
    /// How many units of this asset
    pub quantity: usize,
    /// The value of this asset in USD cents.
    ///
    /// We could use a proper monetary value here like BigDecimal
    /// but those would incur heap allocations. Without additional context
    /// of this service's application, USD cents is a valid way of storing
    /// these values.
    pub price: usize,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct OrderCreatedResponse {
    pub uuid: uuid::Uuid,
}

#[derive(Serialize, Debug)]
pub struct ListAllOrdersResponse {
    pub orders: Vec<Order>,
}
