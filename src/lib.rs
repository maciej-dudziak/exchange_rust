#[derive(Debug, PartialEq, Eq)]
pub struct Order {
    price: i32,
    timestamp: u64,
    quantity: i32,
    instrument: String,
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.price, self.timestamp).cmp(&(other.price, other.timestamp))
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn less() {
        let quantity: i32 = 0;
        let instrument = String::from("ABC");


        let lower_price = Order {
            price: 100,
            timestamp: 2314325421,
            quantity,
            instrument
        };

        // let higher_price = Order {
        //     price: 102,
        //     timestamp: 2314325421,
        //     quantity,
        //     instrument
        // };

        //assert!(lower_price < higher_price)
    }
}
