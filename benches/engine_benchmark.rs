use criterion::{Criterion, black_box, criterion_group, criterion_main};
// Replace `matching_engine` with the actual name of your library crate
use order_book::model::{Order, OrderBook, OrderSide};

/// A helper function to create a realistic, populated order book for our benchmark.
/// This setup work is NOT measured by the benchmark.
fn create_populated_order_book() -> OrderBook {
    let mut order_book = OrderBook::new();

    // Add some resting sell orders (asks) at different price levels
    order_book.process_order(Order {
        side: OrderSide::Sell,
        price: 101,
        quantity: 5,
    });
    order_book.process_order(Order {
        side: OrderSide::Sell,
        price: 102,
        quantity: 10,
    });

    // Add some resting buy orders (bids) at different price levels
    order_book.process_order(Order {
        side: OrderSide::Buy,
        price: 99,
        quantity: 5,
    });
    order_book.process_order(Order {
        side: OrderSide::Buy,
        price: 98,
        quantity: 10,
    });

    order_book
}

/// This is where we define the actual benchmark.
fn benchmark_order_processing(c: &mut Criterion) {
    // Create a benchmark group to organize related benchmarks
    let mut group = c.benchmark_group("MatchingEngine");

    // Benchmark the scenario of processing a new buy order that matches an existing sell order.
    group.bench_function("Process-Buy-Order-With-Match", |b| {
        // `b.iter_with_setup` is perfect for this. It runs the setup code once for each
        // sample, but does not include the setup time in the measurement.
        b.iter_with_setup(
            // --- Setup code ---
            // This closure creates the initial state for each iteration.
            || {
                // We create a fresh, populated order book and the new order we want to process.
                let order_book = create_populated_order_book();
                let new_order = Order {
                    side: OrderSide::Buy,
                    price: 101, // This price will match the existing sell order at 101
                    quantity: 3,
                };
                (order_book, new_order)
            },
            // --- Routine to benchmark ---
            // This closure takes the setup data and runs the code we want to measure.
            |(mut order_book, new_order)| {
                // `black_box` is a hint to the compiler to not optimize away the code inside,
                // ensuring we get an accurate measurement.
                order_book.process_order(black_box(new_order));
            },
        );
    });

    group.finish();
}

// These macros generate the necessary boilerplate to run the benchmarks.
criterion_group!(benches, benchmark_order_processing);
criterion_main!(benches);
