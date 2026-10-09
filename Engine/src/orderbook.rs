use std::collections::{BTreeMap, VecDeque};


#[derive(Debug)]
pub struct OpenOrder {
    pub id: u64,
    pub user_id: u64,
    pub total_qty: u64,
    pub filled_qty: u64,
    pub price: u64
}

#[derive(Debug)]
pub struct Bid {
    pub total_qty: u64,
    pub orders: VecDeque<OpenOrder>
}

#[derive(Debug)]
pub struct Ask {
    pub total_qty: u64,
    pub orders: VecDeque<OpenOrder>
}

#[derive(Debug)]
pub struct Match {
    pub maker: u64,
    pub taker: u64,
    pub price: u64,
    pub qty: u64
}

#[derive(Debug)]
pub struct OrderResponse {
    pub on_book: u64,
    pub filled_qty: u64,
    pub matches: Vec<Match>,
    pub last_traded_price: Option<u64>
}

#[derive(Debug)]
pub struct OrderBook {
    pub asks: BTreeMap<u64, Ask>,
    pub bids: BTreeMap<u64, Bid>,
    pub last_traded_price: Option<u64>
}

#[derive(Debug)]
pub enum Side {
    Ask,
    Bid
}


impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            last_traded_price: None
        }
    }

    pub fn process_order(&mut self, user_id: u64, id: u64, total_qty: u64, price: Option<u64>, side: Side) -> OrderResponse {
        // if total_qty is 0 or less
        if total_qty <= 0 {
            return OrderResponse {
                filled_qty: 0,
                on_book: 0,
                matches: Vec::new(),
                last_traded_price: self.last_traded_price
            };
        }
        let mut remaining_qty = total_qty;
        let mut filled_qty = 0 as u64;
        let mut matches: Vec<Match> =  Vec::new();
        let mut limit_price = 0 as u64;
        let limit_order = match price {
            Some(val) => {
                limit_price = val;
                true
            }
            None => false
        };
        // if limit price is less than or equal to 0
        if limit_order && limit_price <= 0 {
            return OrderResponse {
                filled_qty: 0,
                on_book: 0,
                matches: Vec::new(),
                last_traded_price: self.last_traded_price
            };
        }

        match side {
            Side::Ask => {
                // search in the bids
                loop {
                    if remaining_qty == 0 {
                        break;
                    }
                    let best_bid_price = match self.bids.keys().next_back() {
                        Some(price) => *price,
                        None => 0 as u64 
                    };
                    // if no bids available (not enough liquidity)
                    if best_bid_price == 0 {
                        break;
                    }
                    // if it's a limit order and asked price is higher than best bid price
                    if limit_order && best_bid_price < limit_price {
                        break;
                    }
                    // the best_bid_price must have a corresponding value otherwise it should panic
                    let best_bid = self.bids.get_mut(&best_bid_price).unwrap();
                    let mut completed_bids = 0;
                    let mut trade_happen = false;
                    for bid in best_bid.orders.iter_mut() {
                        if remaining_qty == 0 {
                            break;
                        }
                        let remaining_bid_qty = bid.total_qty - bid.filled_qty;
                        let traded_qty = remaining_bid_qty.min(remaining_qty);
                        bid.filled_qty += traded_qty;
                        remaining_qty -= traded_qty;
                        filled_qty += traded_qty;
                        trade_happen = true;
                        let match_detail = Match {
                            maker: bid.user_id,
                            taker: user_id,
                            price: best_bid_price,
                            qty: traded_qty
                        };
                        matches.push(match_detail);
                        best_bid.total_qty -= traded_qty;
                        // remove the bid if it has been completed
                        if bid.filled_qty == bid.total_qty {
                            completed_bids += 1;
                        }
                    }
                    if trade_happen {
                        self.last_traded_price = Some(best_bid_price);
                    }
                    for _ in 0..completed_bids {
                        best_bid.orders.pop_front();
                    }

                    if best_bid.total_qty == 0 {
                        self.bids.remove(&best_bid_price);
                    }
                }
            }
            Side::Bid => {
                // search in the ask
                loop {
                    if remaining_qty == 0 {
                        break;
                    }
                    let best_ask_price = match self.asks.keys().next() {
                        Some(price) => *price,
                        None => 0 as u64 
                    };
                    // if no asks available (not enough liquidity) 
                    if best_ask_price == 0 {
                        break;
                    }
                    // if limit order and bid price is less than best ask price
                    if limit_order && best_ask_price > limit_price {
                        break;
                    }
                    // the best_ask_price must have a corresponding value otherwise it should panic
                    let best_ask = self.asks.get_mut(&best_ask_price).unwrap();
                    let mut completed_asks = 0;
                    let mut trade_happen = false;
                    for ask in best_ask.orders.iter_mut() {
                        if remaining_qty == 0 {
                            break;
                        }
                        let remaining_ask_qty = ask.total_qty - ask.filled_qty;
                        let traded_qty = remaining_ask_qty.min(remaining_qty);
                        ask.filled_qty += traded_qty;
                        remaining_qty -= traded_qty;
                        filled_qty += traded_qty;
                        trade_happen = true;
                        let match_detail = Match {
                            maker: ask.user_id,
                            taker: user_id,
                            price: best_ask_price,
                            qty: traded_qty
                        };
                        matches.push(match_detail);
                        best_ask.total_qty -= traded_qty;
                        // remove the ask if it has been completed
                        if ask.filled_qty == ask.total_qty {
                            completed_asks += 1;
                        }
                    }
                    if trade_happen {
                        self.last_traded_price = Some(best_ask_price);
                    }
                    for _ in 0..completed_asks {
                        best_ask.orders.pop_front();
                    }

                    if best_ask.total_qty == 0 {
                        self.asks.remove(&best_ask_price);
                    }
                }
            }
        }

        // push in the orderbook if it's a limit order and remaining_qty > 0
        if remaining_qty > 0 && limit_order {
            let new_order = OpenOrder {
                id,
                user_id,
                total_qty,
                filled_qty,
                price: limit_price
            };

            match side {
                Side::Ask => {
                    // add in the ask side
                    let ask = self.asks.entry(limit_price).or_insert(Ask {
                        total_qty: 0,
                        orders: VecDeque::new()
                    });
                    ask.total_qty += remaining_qty;
                    ask.orders.push_back(new_order);
                }
                Side::Bid => {
                    // add in the bid side
                    let bid = self.bids.entry(limit_price).or_insert(Bid {
                        total_qty: 0,
                        orders: VecDeque::new()
                    });
                    bid.total_qty += remaining_qty;
                    bid.orders.push_back(new_order);
                }
            }
        }

        let on_book = if limit_order {
            remaining_qty
        }else {
            0 as u64
        };

        OrderResponse {
            filled_qty,
            on_book,
            matches,
            last_traded_price: self.last_traded_price
        }
    }
}