use axum::{Router, routing::post};
use crossbeam_channel::Sender;

use crate::{http::handlers::handle_create_order, model::Order};

pub fn create_router() -> Router<Sender<Order>> {
    Router::new().route("/orders", post(handle_create_order))
}
