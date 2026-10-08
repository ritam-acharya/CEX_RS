use std::collections::{BTreeMap, VecDeque};



pub struct OpenOrders {
    pub id: u64,
    pub user_id: u64,
    pub total_qty: u64,
    pub filled_qty: u64,
    pub price: u64
}

pub struct Bid {
    pub total_qty: u64,
    pub orders: VecDeque<OpenOrders>
}

pub struct Ask {
    pub total_qty: u64,
    pub orders: VecDeque<OpenOrders>
}

pub struct Match {
    pub maker: u64,
    pub taker: u64,
    pub price: u64,
    pub qty: u64
}

pub struct OrderResponse {
    pub on_book: u64,
    pub filled_qty: u64,
    pub matches: Vec<Match>
}


pub struct OrderBook {
    pub asks: BTreeMap<u64, Ask>,
    pub bids: BTreeMap<u64, Bid>,
    pub last_traded_price: Option<u64>
}


impl OrderBook {
    pub fn new(self) -> Self {
        OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            last_traded_price: None
        }
    }

    pub fn process_order(mut self, user_id: u64, id: u64, total_qty: u64, price: Option<u64>) -> OrderResponse {

        OrderResponse {
            filled_qty: 0,
            on_book: 0,
            matches: Vec::new()
        }
    }
}