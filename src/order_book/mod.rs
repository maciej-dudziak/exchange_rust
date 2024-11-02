mod order;
use order::Order;
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
pub struct OrderBook {
    resting_bid: BinaryHeap<Order>,
    resting_ask: BinaryHeap<Reverse<Order>>
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            resting_bid: BinaryHeap::new(),
            resting_ask: BinaryHeap::new()
        }
    }

    pub fn add_order(&mut self, order: Order, is_buy: bool) {
        if is_buy {
            // TODO match orders here
            let q = self.match_asks(&order);
            if q < order.quantity {
                self.resting_bid.push(order);
            }
        } else {
            // TODO match orders here
            self.match_bids(&order);
        }
        // Avoid it here
        //self.match_orders();
    }

    fn match_asks(&mut self, bid: &Order) -> i32 {
        let mut matched_quantity: i32 = 0;
        while let Some(ask) = self.resting_ask.peek() {
            if ask.0.price <= bid.price {
                let q = bid.quantity.min(ask.0.quantity);
                println!("Matched {} shares of {} at ${}",q, bid.instrument, ask.0.price);
                
                if q == ask.0.quantity {
                    // Filled resting order completely
                    self.resting_ask.pop();
                }

                // Execute on quantity
                matched_quantity += q;
            }
            else 
            {
                break;   
            }
        }
        return matched_quantity;
    }

    fn match_bids(&mut self, ask: &Order) {
        while let Some(bid) = self.resting_bid.peek() {
            if bid.price >= ask.price {
                let matched_quantity = ask.quantity.min(bid.quantity);
                println!("Matched {} shares of {} at ${}",matched_quantity, bid.instrument, bid.price);
                
                if matched_quantity == bid.quantity {
                    // Filled resting order completely
                    self.resting_bid.pop();
                }
                // If didn't fill the order, go further


                // Execute on quantity
            }
            else 
            {
                break;   
            }
        }
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x() {
        let mut ob = OrderBook::new();
        ob.add_order(Order::new(100, 1, 10, String::from("APPL")), true);
        ob.add_order(Order::new(95, 2, 10, String::from("APPL")), true);
        ob.add_order(Order::new(110, 3, 10, String::from("APPL")), false);
        ob.add_order(Order::new(115, 3, 10, String::from("APPL")), false);

        ob.add_order(Order::new(99, 3, 10, String::from("APPL")), false);

        println!("OrderBook is {:?}", ob)
    }
}