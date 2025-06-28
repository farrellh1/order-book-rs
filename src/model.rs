use std::{cmp, collections::BTreeMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Order {
    pub side: OrderSide,
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trade {
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderBook {
    pub bids: BTreeMap<u64, Vec<Order>>,
    pub asks: BTreeMap<u64, Vec<Order>>,
    pub trades: Vec<Trade>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            trades: Vec::new(),
        }
    }

    pub fn process_order(&mut self, mut order: Order) {
        match order.side {
            OrderSide::Buy => self.process_buy_order(&mut order),
            OrderSide::Sell => self.process_sell_order(&mut order),
        }
    }

    pub fn process_buy_order(&mut self, order: &mut Order) {
        // This loop continues as long as the incoming buy order has quantity
        // and there are asks on the book that are cheap enough to match.
        while order.quantity > 0 {
            // Find the best (lowest) priced ask order.
            let best_ask_price = match self.asks.first_key_value() {
                Some((&price, _)) if order.price >= price => price,
                _ => {
                    // If no asks exist, or none are cheap enough, stop matching.
                    break;
                }
            };

            // Get the list of orders at that best price.
            let asks_at_best_price = self.asks.get_mut(&best_ask_price).unwrap();

            // --- Inner loop: Process all orders at this single price level ---
            // This part is similar to your original logic.
            for ask_order in asks_at_best_price.iter_mut() {
                if order.quantity == 0 {
                    break; // Incoming order is fully filled.
                }

                let trade_quantity = cmp::min(order.quantity, ask_order.quantity);

                self.trades.push(Trade {
                    price: ask_order.price,
                    quantity: trade_quantity,
                });

                order.quantity -= trade_quantity;
                ask_order.quantity -= trade_quantity;
            }

            // After iterating, remove any orders that were fully filled.
            asks_at_best_price.retain(|o| o.quantity > 0);

            // If the entire price level is now empty, remove it from the book.
            // This is crucial so the next iteration of the outer loop sees the *new* best price.
            if asks_at_best_price.is_empty() {
                self.asks.remove(&best_ask_price);
            }
        }

        // If there's still quantity left in the buy order after matching,
        // it becomes a resting order in the bid book.
        if order.quantity > 0 {
            self.bids
                .entry(order.price)
                .or_default()
                .push(order.clone());
        }
    }

    pub fn process_sell_order(&mut self, order: &mut Order) {
        while order.quantity > 0 {
            // Find the best (highest) priced bid order.
            let best_bid_price = match self.bids.last_key_value() {
                Some((&price, _)) if order.price <= price => price,
                _ => {
                    // No bids, or none are high enough.
                    break;
                }
            };

            let bids_at_best_price = self.bids.get_mut(&best_bid_price).unwrap();

            for bid_order in bids_at_best_price.iter_mut() {
                if order.quantity == 0 {
                    break;
                }

                let trade_quantity = cmp::min(order.quantity, bid_order.quantity);

                self.trades.push(Trade {
                    price: bid_order.price,
                    quantity: trade_quantity,
                });

                order.quantity -= trade_quantity;
                bid_order.quantity -= trade_quantity;
            }

            bids_at_best_price.retain(|o| o.quantity > 0);

            if bids_at_best_price.is_empty() {
                self.bids.remove(&best_bid_price);
            }
        }

        // If there's still quantity left in the sell order,
        // it becomes a resting order in the ask book.
        if order.quantity > 0 {
            self.asks
                .entry(order.price)
                .or_default()
                .push(order.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_match() {
        let mut order_book = OrderBook::new();

        order_book.process_order(Order {
            side: OrderSide::Buy,
            price: 100,
            quantity: 1,
        });
        order_book.process_order(Order {
            side: OrderSide::Sell,
            price: 100,
            quantity: 1,
        });

        assert_eq!(order_book.trades.len(), 1);
        assert_eq!(order_book.trades[0].price, 100);
        assert_eq!(order_book.trades[0].quantity, 1);
        assert!(order_book.bids.is_empty());
        assert!(order_book.asks.is_empty());
    }

    #[test]
    fn test_partial_match() {
        let mut order_book = OrderBook::new();

        order_book.process_order(Order {
            side: OrderSide::Buy,
            price: 100,
            quantity: 2,
        });
        order_book.process_order(Order {
            side: OrderSide::Sell,
            price: 100,
            quantity: 1,
        });

        assert_eq!(order_book.trades.len(), 1);
        assert_eq!(order_book.trades[0].quantity, 1);
        // The buy order should have 1 quantity remaining.
        assert_eq!(order_book.bids.get(&100).unwrap()[0].quantity, 1);
        assert!(order_book.asks.is_empty());
    }

    #[test]
    fn test_no_match() {
        let mut order_book = OrderBook::new();

        order_book.process_order(Order {
            side: OrderSide::Buy,
            price: 100,
            quantity: 1,
        });
        order_book.process_order(Order {
            side: OrderSide::Sell,
            price: 101,
            quantity: 1,
        });

        assert_eq!(order_book.trades.len(), 0);
        assert_eq!(order_book.bids.get(&100).unwrap().len(), 1);
        assert_eq!(order_book.asks.get(&101).unwrap().len(), 1);
    }

    #[test]
    fn test_match_across_multiple_price_levels() {
        let mut order_book = OrderBook::new();

        // Add two sell orders at different prices
        order_book.process_order(Order {
            side: OrderSide::Sell,
            price: 101,
            quantity: 5,
        });
        order_book.process_order(Order {
            side: OrderSide::Sell,
            price: 102,
            quantity: 5,
        });

        // A large buy order comes in that should fill both sell orders
        order_book.process_order(Order {
            side: OrderSide::Buy,
            price: 102,
            quantity: 12,
        });

        // Check that two trades were created
        assert_eq!(order_book.trades.len(), 2);

        // Check the details of the first trade (at the best price of 101)
        assert_eq!(order_book.trades[0].price, 101);
        assert_eq!(order_book.trades[0].quantity, 5);

        // Check the details of the second trade (at the next best price of 102)
        assert_eq!(order_book.trades[1].price, 102);
        assert_eq!(order_book.trades[1].quantity, 5);

        // The entire ask book should be empty now
        assert!(order_book.asks.is_empty());

        // A new buy order for the remaining quantity (12 - 5 - 5 = 2) should be in the bid book
        assert_eq!(order_book.bids.get(&102).unwrap()[0].quantity, 2);
    }
}
