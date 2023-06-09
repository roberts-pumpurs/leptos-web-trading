use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message, Recipient};
use rust_decimal_macros::dec;
use trading_types::common::{Order, Size, Tick, TraderId};
use trading_types::from_server::TickData;

pub mod messages {

    use super::*;

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub struct PlaceOrder(pub TraderId, pub Order);

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub struct RegisterForUpdates(pub TraderId, pub Recipient<TickDataUpdate>);

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub enum TickDataUpdate {
        SetRefresh(Vec<TickData>),
        SingleUpdate(TickData),
    }
}

pub struct MarketActor {
    order_book: HashMap<Tick, OrderBookRange>,
    update_subscribers: HashMap<TraderId, Recipient<messages::TickDataUpdate>>,
}

impl Actor for MarketActor {
    type Context = Context<Self>;
}

impl Handler<messages::PlaceOrder> for MarketActor {
    type Result = ();

    fn handle(&mut self, msg: messages::PlaceOrder, _ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Received order");

        if let Some(ob) = self.order_book.get_mut(&msg.1.tick) {
            match msg.1.side {
                trading_types::common::Side::Back => {
                    ob.back.push((msg.0, msg.1.size));
                }
                trading_types::common::Side::Lay => {
                    ob.lay.push((msg.0, msg.1.size));
                }
            }
            // TODO Reduce the order book row by matching backs and lays

            let tick_data = compress_single_order(ob);
            self.update_listeners(messages::TickDataUpdate::SingleUpdate(tick_data), _ctx);
        }
    }
}

impl Handler<messages::RegisterForUpdates> for MarketActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: messages::RegisterForUpdates,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        tracing::info!(msg = ?msg, "Registering for market updates");

        let tick_data = self.order_book.values().map(compress_single_order).collect::<Vec<_>>();
        let update_msg = messages::TickDataUpdate::SetRefresh(tick_data);
        msg.1.do_send(update_msg);
        self.update_subscribers.insert(msg.0, msg.1);
    }
}

fn compress_single_order(value: &OrderBookRange) -> TickData {
    let compressed_back = value.back.iter().map(|x| &x.1).fold(Size(dec!(0)), |acc, i| acc + i);
    let compressed_lay = value.lay.iter().map(|x| &x.1).fold(Size(dec!(0)), |acc, i| acc + i);
    TickData { tick: value.tick.clone(), back: compressed_back, lay: compressed_lay }
}

impl MarketActor {
    pub fn new() -> Self {
        let mut order_book = HashMap::new();
        for tick in Tick::all() {
            order_book.insert(tick.clone(), OrderBookRange::new(tick));
        }
        Self { order_book, update_subscribers: HashMap::new() }
    }

    pub fn update_listeners(&self, msg: messages::TickDataUpdate, _ctx: &mut Context<Self>) {
        for (_, recp) in self.update_subscribers.iter() {
            recp.do_send(msg.clone());
        }
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
