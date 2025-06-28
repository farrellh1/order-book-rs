#![allow(unused)]
mod model;

use std::{
    cmp,
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use axum::Router;
use order_book::{http::routes::create_router, model::OrderBook};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let order_book = Arc::new(Mutex::new(OrderBook::new()));
    let app = create_router(order_book);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
