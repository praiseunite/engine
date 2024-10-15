//Starting the engine app.
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Price {
    integral : u64,
    fractional : u64,
    scaler : u64,
}

impl Price {
    fn new(price : f64) -> Price {
        let scaler = 100000;
        let integral = price as u64;
        // let fractional = ((price - integral as f64) * scaler as f64) as u64;
        let fractional = ((price % 1.0) * scaler as f64) as u64;
        Price {
            integral,
            fractional,
            scaler,
        }
    }
    
}

struct Limit {
    price : Price,
    orders : Vec<Order>,
}

impl Limit {
    fn new(price : f64) -> Limit { 
        Limit {
            price : Price::new(price),  // call Price::new
            orders : Vec::new(), // empty vector
        }
    }
}

struct Order {
    size : f64,
    bid_or_ask : BidOrAsk,
}

impl  Order {
    fn new(size: f64, bid_or_ask: BidOrAsk) -> Order {
        Order {
            size,
            bid_or_ask,
        }
    }
    
}

fn main() {
    let price = Price::new(50.5);
    println!("{:?}", price);
}
