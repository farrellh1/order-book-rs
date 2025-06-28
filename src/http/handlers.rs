use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use crossbeam_channel::Sender;

use crate::model::Order;

pub async fn handle_create_order(
    State(sender): State<Sender<Order>>,
    Json(order): Json<Order>,
) -> impl IntoResponse {
    println!("Received order: {:?}", order);

    if let Ok(_) = sender.send(order.clone()) {
        println!("Order sent to channel: {:?}", order);
        return (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({"status": "Order accepted"})),
        );
    } else {
        eprintln!("Failed to send order to channel");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to process order"})),
        );
    }
}
