#![allow(unused)]
mod model;

use std::{
    cmp,
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use axum::Router;
use crossbeam_channel::unbounded;
use order_book::{
    http::routes::create_router,
    model::{Order, OrderBook},
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let mut order_book = OrderBook::new();
    let (s, r) = unbounded();
    tokio::task::spawn_blocking(move || {
        while let Ok(order) = r.recv() {
            order_book.process_order(order);
            println!("Order processed: {:?}", order_book);
        }
    });
    let app = create_router().with_state(s);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
