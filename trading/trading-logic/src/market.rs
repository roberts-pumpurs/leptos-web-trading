use std::collections::HashMap;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, Recipient};
use nanoid::nanoid;
use rust_decimal_macros::dec;
use trading_types::common::{Order, Size, Tick, TraderId, RequestId};
use trading_types::from_server::TickData;

use crate::bot::BotActor;

pub mod messages {

    use super::*;

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub struct PlaceOrder {
        pub trader: TraderId,
        pub request_id: RequestId,
        pub order: Order,
    }

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub struct SpawnBot;

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub struct RegisterTrader(pub TraderId, pub Recipient<TickDataUpdate>);

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub enum TickDataUpdate {
        SetRefresh(Vec<TickData>),
        SingleUpdate(TickData),
    }
}

pub struct MarketActor {
    order_book: HashMap<Tick, OrderBookRange>,
    update_subscribers: HashMap<TraderId, InternalTraderState>,
    bots: Vec<Addr<BotActor>>,
}

struct InternalTraderState {
    recp: Recipient<messages::TickDataUpdate>,
    matched: Size,
    // TODO add open orders here
    // TODO add matched orders here
    // TODO other metadata, like exposure, etc
}

impl Actor for MarketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Reset the game every minute
        ctx.run_interval(std::time::Duration::from_secs(60), |act, ctx| {
            for (_key, val) in act.order_book.iter_mut() {
                val.clear();
            }
            let update_msg = act.tick_data_refresh_msg();
            act.update_listeners(update_msg, ctx);
        });
    }
}

impl Handler<messages::PlaceOrder> for MarketActor {
    type Result = ();

    fn handle(&mut self, mut msg: messages::PlaceOrder, _ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Received order");

        if let Some(obr) = self.order_book.get_mut(&msg.order.tick) {
            match msg.order.side {
                trading_types::common::Side::Back => {
                    // TODO match against lays
                    for (lay_trader_id, lay_req_id, lay_order_size) in obr.lay.iter_mut() {
                        match msg.order.size.cmp(lay_order_size) {
                            std::cmp::Ordering::Less => {
                                lay_order_size.0 -= msg.order.size.0;
                                msg.order.size.0 = dec!(0);
                            },
                            std::cmp::Ordering::Equal => {
                                msg.order.size.0 = dec!(0);
                                lay_order_size.0 = dec!(0);

                            },
                            std::cmp::Ordering::Greater => {
                                msg.order.size.0 -= lay_order_size.0;
                                lay_order_size.0 = dec!(0);
                            },
                        }
                        // TODO send matched message to lay_trader_id

                        if msg.order.size.0 == dec!(0) {
                            break;
                        }
                    }
                    if msg.order.size.0 > dec!(0) {
                        obr.back.push((msg.trader, msg.request_id, msg.order.size));
                    }
                }
                trading_types::common::Side::Lay => {
                    for (back_trader_id, back_req_id, back_order_size) in obr.back.iter_mut() {
                        match msg.order.size.cmp(back_order_size) {
                            std::cmp::Ordering::Less => {
                                back_order_size.0 -= msg.order.size.0;
                            },
                            std::cmp::Ordering::Equal => {
                                msg.order.size.0 = dec!(0);
                                back_order_size.0 = dec!(0);

                            },
                            std::cmp::Ordering::Greater => {
                                msg.order.size.0 -= back_order_size.0;
                                back_order_size.0 = dec!(0);
                            },
                        }
                        // TODO send matched message to lay_trader_id

                        if msg.order.size.0 == dec!(0) {
                            break;
                        }
                    }
                    if msg.order.size.0 > dec!(0) {
                        obr.lay.push((msg.trader, msg.request_id, msg.order.size));
                    }
                }
            }

            let tick_data = compress_order_book_range(obr);
            self.update_listeners(messages::TickDataUpdate::SingleUpdate(tick_data), _ctx);
        }
    }
}

impl Handler<messages::RegisterTrader> for MarketActor {
    type Result = ();

    fn handle(&mut self, msg: messages::RegisterTrader, _ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Registering for market updates");

        let update_msg = self.tick_data_refresh_msg();
        msg.1.do_send(update_msg);
        let state = InternalTraderState { recp: msg.1, matched: Size(dec!(0)) };
        self.update_subscribers.insert(msg.0, state);
    }
}

impl Handler<messages::SpawnBot> for MarketActor {
    type Result = ();

    fn handle(&mut self, msg: messages::SpawnBot, ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Spawning bot");
        let bot = nanoid!(5) + "-bot";
        let bot = crate::bot::BotActor::new(ctx.address(), TraderId(bot)).start();
        self.bots.push(bot);
    }
}

fn compress_order_book_range(value: &mut OrderBookRange) -> TickData {
    let lays = value.lay.drain_filter(|(a, b, size)| {
        size.0 > dec!(0)
    }).collect::<Vec<_>>();
    value.lay = lays;
    let backs: Vec<(TraderId, RequestId, Size)> = value.back.drain_filter(|(a, b, size)| {
        size.0 > dec!(0)
    }).collect::<Vec<_>>();
    value.back = backs;

    let compressed_back = value.back.iter().map(|x| &x.2).fold(Size(dec!(0)), |acc, i| acc + i);
    let compressed_lay = value.lay.iter().map(|x| &x.2).fold(Size(dec!(0)), |acc, i| acc + i);
    TickData {
        total_liquidity: Size(compressed_back.0 + compressed_lay.0),
        tick: value.tick,
        back: compressed_back,
        lay: compressed_lay,
        matched_liquidity: Size(dec!(0)),
    }
}

impl MarketActor {
    pub fn new() -> Self {
        let mut order_book = HashMap::new();
        for tick in Tick::all() {
            order_book.insert(tick, OrderBookRange::new(tick));
        }

        Self { order_book, update_subscribers: HashMap::new(), bots: Vec::new() }
    }

    pub fn update_listeners(&self, msg: messages::TickDataUpdate, _ctx: &mut Context<Self>) {
        for (_, trader) in self.update_subscribers.iter() {
            trader.recp.do_send(msg.clone());
        }
    }

    fn tick_data_refresh_msg(&mut self) -> messages::TickDataUpdate {
        let mut tick_data = self.order_book.values_mut().map(compress_order_book_range).collect::<Vec<_>>();
        tick_data.sort_by(|a, b| match a.tick.0 < b.tick.0 {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        });

        messages::TickDataUpdate::SetRefresh(tick_data)
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct OrderBookRange {
    back: Vec<(TraderId, RequestId, Size)>,
    lay: Vec<(TraderId, RequestId, Size)>,
    tick: Tick,
}

impl OrderBookRange {
    fn new(tick: Tick) -> Self {
        Self { back: Vec::new(), lay: Vec::new(), tick }
    }

    fn clear(&mut self) {
        self.back.clear();
        self.lay.clear();
    }
}
