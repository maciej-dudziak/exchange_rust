#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Order {
    pub price: i32,
    timestamp: u64,
    pub quantity: i32,
    pub instrument: String,
}

impl Order {
    pub fn new(price: i32, timestamp: u64, quantity: i32, instrument: String) -> Self {
        Order {price, timestamp, quantity, instrument}
    }
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
    fn smaller_by_price() {
        let timestamp: u64 = 21783923;
        let o1 = Order {
            price: 100,
            timestamp,
            quantity: 10,
            instrument: String::from("APPL")
        };
        let o2 = Order {
            price: o1.price + 5,
            timestamp,
            quantity: 10,
            instrument: String::from("APPL")
        };
        assert!(o1 < o2);
    }

    #[test]
    fn smaller_by_timestamp() {
        let timestamp: u64 = 21783923;
        let o1 = Order {
            price: 100,
            timestamp,
            quantity: 10,
            instrument: String::from("APPL")
        };
        let o2 = Order {
            price: o1.price,
            timestamp: timestamp + 5,
            quantity: 10,
            instrument: String::from("APPL")
        };
        assert!(o1 < o2);
    }

    #[test]
    fn equal() {
        let o1 = Order {
            price: 100,
            timestamp: 21346526,
            quantity: 10,
            instrument: String::from("APPL")
        };
        let o2 = o1.clone();
        assert!(o1 == o2);
    }

    #[test]
    fn not_equal() {
        let o1 = Order {
            price: 100,
            timestamp: 21346526,
            quantity: 10,
            instrument: String::from("APPL")
        };
        let o2 = Order{
            price: 100,
            timestamp: 21346526,
            quantity: 15,
            instrument: String::from("APPL")
        };
        let o3 = Order{
            price: 100,
            timestamp: 21346526,
            quantity: 15,
            instrument: String::from("MSFT")
        };
        assert!(!(o1 == o2));
        assert!(!(o1 == o3));
    }
}
