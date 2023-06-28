use actix_web::{
    get, post,
    web::{self, Json},
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    api::forms::{
        ListAllOrdersResponse, OrderCreatedResponse, PartialOrder,
    },
    order::{Order, OrderSide},
    storage::OrderStorage,
    Result,
};

async fn insert_order(
    storage: &OrderStorage,
    partial_order: PartialOrder,
    order_side: OrderSide,
) -> OrderCreatedResponse {
    let uuid = Uuid::new_v4();

    let order = Order::from_form(uuid, partial_order, order_side);

    storage.insert(order).await;

    OrderCreatedResponse { uuid }
}

#[instrument(skip(storage))]
#[get("/orders")]
async fn list_all(
    storage: web::Data<OrderStorage>,
) -> Result<Json<ListAllOrdersResponse>> {
    let orders = storage.list_all().await;

    let response = ListAllOrdersResponse { orders };

    Ok(Json(response))
}

#[instrument(skip(storage))]
#[post("/orders/bids")]
pub async fn create_bid(
    web::Json(partial_order): web::Json<PartialOrder>,
    storage: web::Data<OrderStorage>,
) -> Result<Json<OrderCreatedResponse>> {
    let response =
        insert_order(&storage, partial_order, OrderSide::Bid).await;

    Ok(Json(response))
}

#[instrument(skip(storage))]
#[post("/orders/asks")]
pub async fn create_ask(
    web::Json(partial_order): web::Json<PartialOrder>,
    storage: web::Data<OrderStorage>,
) -> Result<Json<OrderCreatedResponse>> {
    let response =
        insert_order(&storage, partial_order, OrderSide::Ask).await;

    Ok(Json(response))
}
