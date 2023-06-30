use actix_web::{
    delete, get, post,
    web::{self, Json},
    HttpResponse,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    api::forms::{
        ListAllOrdersResponse, OrderCreatedResponse, OrderToCancel,
        PartialOrder,
    },
    order::{Order, OrderSide},
    Result, SharedOrderRepositoryImpl,
};

/// Utility function to insert a new order into the storage
async fn insert_order(
    storage: &SharedOrderRepositoryImpl,
    partial_order: PartialOrder,
    order_side: OrderSide,
) -> Result<OrderCreatedResponse> {
    let uuid = Uuid::new_v4();

    let order = Order::from_form(uuid, partial_order, order_side);

    storage.insert(order).await?;

    Ok(OrderCreatedResponse { uuid })
}

#[instrument(skip(storage))]
#[get("")]
pub async fn list_all(
    storage: web::Data<SharedOrderRepositoryImpl>,
) -> Result<Json<ListAllOrdersResponse>> {
    let orders = storage.list_all().await?;

    let response = ListAllOrdersResponse { orders };

    Ok(Json(response))
}

#[instrument(skip(storage))]
#[post("/bids")]
pub async fn create_bid(
    web::Json(partial_order): web::Json<PartialOrder>,
    storage: web::Data<SharedOrderRepositoryImpl>,
) -> Result<Json<OrderCreatedResponse>> {
    let response =
        insert_order(&storage, partial_order, OrderSide::Bid).await?;

    Ok(Json(response))
}

#[instrument(skip(storage))]
#[post("/asks")]
pub async fn create_ask(
    web::Json(partial_order): web::Json<PartialOrder>,
    storage: web::Data<SharedOrderRepositoryImpl>,
) -> Result<Json<OrderCreatedResponse>> {
    let response =
        insert_order(&storage, partial_order, OrderSide::Ask).await?;

    Ok(Json(response))
}

#[instrument(skip(storage))]
#[delete("")]
pub async fn cancel_order(
    web::Json(to_cancel): web::Json<OrderToCancel>,
    storage: web::Data<SharedOrderRepositoryImpl>,
) -> Result<HttpResponse> {
    let OrderToCancel { uuid } = to_cancel;

    storage.remove(&uuid).await?;

    // Return No-Content
    Ok(HttpResponse::Ok().finish())
}
