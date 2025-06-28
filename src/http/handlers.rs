use std::sync::{Arc, Mutex};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::model::{Order, OrderBook};

pub async fn handle_create_order(
    State(order_book): State<Arc<Mutex<OrderBook>>>,
    Json(order): Json<Order>,
) -> impl IntoResponse {
    println!("Received order: {:?}", order);

    let mut book = order_book.lock().unwrap();
    book.process_order(order);
    (
        StatusCode::CREATED,
        Json(serde_json::json!({"message": "Order processed successfully", "trades": book.trades})),
    )
}
