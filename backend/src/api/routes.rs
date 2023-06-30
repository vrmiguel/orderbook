use actix_web::{
    get, post,
    web::{self, Json},
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    api::forms::{
        ListAllOrdersResponse, OrderCreatedResponse, OrderToCancel,
        PartialOrder,
    },
    error::Error,
    order::{Order, OrderSide},
    storage::OrderStorage,
    Result,
};

/// Utility function to insert a new order into the storage
fn insert_order(
    storage: &OrderStorage,
    partial_order: PartialOrder,
    order_side: OrderSide,
) -> OrderCreatedResponse {
    let uuid = Uuid::new_v4();

    let order = Order::from_form(uuid, partial_order, order_side);

    storage.insert(order);

    OrderCreatedResponse { uuid }
}

#[instrument(skip(storage))]
#[get("")]
pub async fn list_all(
    storage: web::Data<OrderStorage>,
) -> Result<Json<ListAllOrdersResponse>> {
    let orders = storage.list_all().await;

    let response = ListAllOrdersResponse { orders };

    Ok(Json(response))
}

#[instrument(skip(storage))]
#[post("/bids")]
pub async fn create_bid(
    web::Json(partial_order): web::Json<PartialOrder>,
    storage: web::Data<OrderStorage>,
) -> Result<Json<OrderCreatedResponse>> {
    let response = insert_order(&storage, partial_order, OrderSide::Bid);

    Ok(Json(response))
}

#[instrument(skip(storage))]
#[post("/asks")]
pub async fn create_ask(
    web::Json(partial_order): web::Json<PartialOrder>,
    storage: web::Data<OrderStorage>,
) -> Result<Json<OrderCreatedResponse>> {
    let response = insert_order(&storage, partial_order, OrderSide::Ask);

    Ok(Json(response))
}

#[instrument(skip(storage))]
#[post("/asks")]
pub async fn create_order(
    web::Json(to_cancel): web::Json<OrderToCancel>,
    storage: web::Data<OrderStorage>,
) -> Result<Json<Order>> {
    let OrderToCancel { uuid } = to_cancel;

    let removed_order = storage.remove(&uuid).ok_or(Error::NotFound)?;

    Ok(Json(removed_order))
}
