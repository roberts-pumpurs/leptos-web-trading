use std::collections::{HashMap, HashSet};

use actix::{Actor, Context, Message, Handler};
use trading_types::common::{Size, Tick, TraderId};

pub mod messages {
    use actix::Recipient;
    use trading_types::{common::Order, from_server::{TickData, MatchInfo}};

    use super::*;

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub struct PlaceOrder(pub TraderId, pub Order);

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub struct RegisterForUpdates(pub Recipient<TickDataUpdate>);

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub enum TickDataUpdate {
        SetRefresh(Vec<TickData>),
        SingleUpdate(TickData),
        MatchInfo(MatchInfo),
    }
}

pub struct MarketActor {
    order_book: HashMap<Tick, OrderBookRange>,
    update_subscribers: HashSet<TraderId>,
}


impl Actor for MarketActor {
    type Context = Context<Self>;
}

impl Handler<messages::PlaceOrder> for MarketActor {
    type Result = ();

    fn handle(&mut self, msg: messages::PlaceOrder, _ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Received order");
        ()
    }
}


impl MarketActor {
    pub fn new() -> Self {
        let mut order_book = HashMap::new();
        for tick in Tick::all() {
            order_book.insert(tick.clone(), OrderBookRange::new(tick));
        }
        Self { order_book, update_subscribers: HashSet::new() }
    }
}




#[derive(Message)]
#[rtype(result = "()")]
struct OrderBookRange {
    back: Vec<(TraderId, Size)>,
    lay: Vec<(TraderId, Size)>,
    tick: Tick,
}

impl OrderBookRange {
    fn new(tick: Tick) -> Self {
        Self { back: Vec::new(), lay: Vec::new(), tick }
    }
}
