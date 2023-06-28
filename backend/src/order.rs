use serde::Serialize;
use uuid::Uuid;

use crate::api::forms::PartialOrder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderSide {
    Bid,
    Ask,
}

pub type Cents = usize;

/// A bid or ask order for a single asset of a financial market.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Order {
    // TODO: remove UUID from here?
    /// Order's unique identifier
    pub id: uuid::Uuid,
    /// How many units of this asset
    pub quantity: usize,
    /// The value of this asset in USD cents.
    ///
    /// We could use a proper monetary value here like BigDecimal
    /// but those would incur heap allocations. Without additional context
    /// of this service's application, USD cents is a valid way of storing
    /// these values.
    pub price: Cents,
    /// If this is a bid or an ask order
    pub side: OrderSide,
}

impl Order {
    /// Builds a new order from an input form.
    ///
    /// Assings a UUID to the created order.
    pub fn from_form(
        uuid: Uuid,
        PartialOrder { quantity, price }: PartialOrder,
        order_side: OrderSide,
    ) -> Self {
        Order {
            id: uuid,
            quantity,
            price,
            side: order_side,
        }
    }
}
