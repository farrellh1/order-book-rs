use std::sync::{Arc, Mutex};

use axum::{Router, routing::post};

use crate::{http::handlers::handle_create_order, model::OrderBook};

pub fn create_router(order_book: Arc<Mutex<OrderBook>>) -> Router {
    Router::new()
        .route("/orders", post(handle_create_order))
        .with_state(order_book)
}
