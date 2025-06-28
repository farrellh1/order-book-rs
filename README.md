# High-Performance Limit Order Book Matching Engine in Rust

This repository contains a high-performance, in-memory limit order book (LOB) matching engine built in Rust. The project is designed to simulate the core functionality of a modern financial exchange, focusing on low-latency order processing and high throughput.

This project was built to demonstrate a deep understanding of systems engineering, concurrency patterns, and the performance-critical requirements of high-frequency trading (HFT) infrastructure.

## Architectural Implementations

This repository contains two distinct implementations on separate branches, each demonstrating a different concurrency model and its associated trade-offs.

### 1. `mutex-impl` (Simple Shared-State Model)

* **Architecture:** This implementation uses a simple and direct approach where the `OrderBook` struct is wrapped in an `Arc<Mutex<>>`. All Axum API handlers share this state and acquire a lock before processing an order.
* **Pros:** Simple to implement and reason about for basic use cases.
* **Cons:** The single mutex creates a major performance bottleneck under high load, as all incoming orders must be processed sequentially. This is not suitable for a real HFT environment.

### 2. `main` (High-Performance Message-Passing Model)

* **Architecture:** This is the production-ready implementation. It decouples the API from the matching engine using `crossbeam` channels. A dedicated thread owns the `OrderBook`, and the Axum API handlers' only job is to instantly place incoming orders into a channel.
* **Pros:** Extremely high throughput and low API latency. The server can ingest a massive volume of orders without blocking. This lock-free design is a core pattern used in real-world HFT systems.
* **Cons:** More complex to set up, requiring careful management of the processing thread and channels.

## Features

* **Accurate Matching Logic:** Correctly matches incoming buy and sell orders based on price-time priority.
* **Partial Fills:** Handles orders of different quantities, correctly executing partial fills and leaving the remaining quantity on the book.
* **Book "Walking":** An incoming order will "walk the book," matching against multiple price levels until it is either completely filled or there is no more liquidity available at a compatible price.
* **Asynchronous API:** A non-blocking API built with `axum` for submitting new orders.

---

## Performance Benchmarks

Performance is a key feature of this project. The core matching logic was benchmarked using `criterion.rs`.

* **Task:** Processing a single limit order against a moderately populated book.
* **Result:** Average processing time: ~170 nanoseconds per order average on an M3 PRO.

*This benchmark demonstrates that the core logic is extremely fast and that the primary latency in a real system would come from network I/O, not the matching engine itself.*

---

## Tech Stack

* **Core Language:** [**Rust**](https://www.rust-lang.org/)
* **Web Framework:** [**`axum`**](https://github.com/tokio-rs/axum) for the asynchronous API layer.
* **Concurrency:**
    * `Arc<Mutex<>>` on the `mutex-impl`.
    * `crossbeam-channel` on the `main` branch for high-performance message passing.
* **Data Structures:** `BTreeMap` is used for the bid and ask books to ensure that price levels are always sorted, allowing for efficient lookup of the best price.
* **Benchmarking:** [**`criterion.rs`**](https://github.com/bheisler/criterion.rs)

---

## Getting Started

### Prerequisites

* Rust and Cargo installed.

### Installation & Running

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/farrellh1/order-book-rs.git
    cd order-book-rs
    ```

2.  **Switch to the desired branch:**
    * For the simple version: `git checkout mutex-impl`
    * For the high-performance version: `git checkout main`

3.  **Run the API server:**
    ```bash
    cargo run --release
    ```
    The server will start on `http://127.0.0.1:3000`.

### API Usage

You can submit a new order to the engine using `curl`:

**Submit a Buy Order:**
```bash
curl -X POST \
  http://127.0.0.1:3000/orders \
  -H 'Content-Type: application/json' \
  -d '{
        "side": "Buy",
        "price": 101,
        "quantity": 10
      }'
```
**Submit a Sell Order:**

```bash
curl -X POST \
  http://127.0.0.1:3000/orders \
  -H 'Content-Type: application/json' \
  -d '{
        "side": "Sell",
        "price": 101,
        "quantity": 5
      }'
```
