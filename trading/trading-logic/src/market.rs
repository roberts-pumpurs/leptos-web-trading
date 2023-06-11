use std::collections::HashMap;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, Recipient};
use nanoid::nanoid;
use rust_decimal_macros::dec;
use trading_types::common::{Order, RequestId, Size, Tick, TraderId};
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
    subscribers: HashMap<TraderId, InternalTraderState>,
    bots: Vec<Addr<BotActor>>,
}

struct InternalTraderState {
    recp: Recipient<messages::TickDataUpdate>,
    open_orders: HashMap<RequestId, Order>,
    matched_orders: HashMap<RequestId, Order>,
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

    fn handle(&mut self, msg: messages::PlaceOrder, _ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Received order");

        if let Some(obr) = self.order_book.get_mut(&msg.order.tick) {
            let matched_opposing = match msg.order.side {
                trading_types::common::Side::Back => {
                    let (leftover_size, matched_opposing) =
                        match_orders_back(msg.order.size, &mut obr.lay);
                    match leftover_size {
                        Some(leftover_size) => {
                            if let Some(v) = self.subscribers.get_mut(&msg.trader) {
                                v.open_orders.insert(msg.request_id.clone(), msg.order.clone());
                            }
                            obr.back.push((msg.trader, msg.request_id, leftover_size));
                        }
                        None => (),
                    }
                    matched_opposing
                }
                trading_types::common::Side::Lay => {
                    let (leftover_size, matched_opposing) =
                        match_orders_back(msg.order.size, &mut obr.back);
                    match leftover_size {
                        Some(leftover_size) => {
                            if let Some(v) = self.subscribers.get_mut(&msg.trader) {
                                v.open_orders.insert(msg.request_id.clone(), msg.order.clone());
                            }
                            obr.lay.push((msg.trader, msg.request_id, leftover_size));
                        }
                        None => (),
                    }
                    matched_opposing
                }
            };
            for (trader_id, request_id, leftover_size, fully_matched) in matched_opposing {
                if let Some(v) = self.subscribers.get_mut(&trader_id) {
                    if fully_matched {
                        if let Some(order) = v.open_orders.remove(&request_id) {
                            v.matched_orders.insert(request_id, order);
                        }
                    } else {
                        v.open_orders
                            .get_mut(&request_id)
                            .map(|order| order.size.0 = leftover_size.0);
                    }
                }
                // TODO Update the trader with the info that his bets have been matched
            }

            let tick_data = compress_order_book_range(obr);
            self.update_listeners(messages::TickDataUpdate::SingleUpdate(tick_data), _ctx);
        }
    }
}

fn match_orders_back(
    mut order_size: Size,
    opposing: &mut Vec<(TraderId, RequestId, Size)>,
) -> (Option<Size>, Vec<(TraderId, RequestId, Size, bool)>) {
    let mut matched_opposing = vec![];
    for (opposing_trader_id, opposing_req_id, opposing_order_size) in opposing.iter_mut() {
        match order_size.cmp(opposing_order_size) {
            std::cmp::Ordering::Less => {
                opposing_order_size.0 -= order_size.0;
                order_size.0 = dec!(0);
            }
            std::cmp::Ordering::Equal => {
                order_size.0 = dec!(0);
                opposing_order_size.0 = dec!(0);
            }
            std::cmp::Ordering::Greater => {
                order_size.0 -= opposing_order_size.0;
                opposing_order_size.0 = dec!(0);
            }
        }
        matched_opposing.push((
            opposing_trader_id.clone(),
            opposing_req_id.clone(),
            *opposing_order_size,
            opposing_order_size.0 == dec!(0),
        ));

        if order_size.0 == dec!(0) {
            break
        }
    }
    *opposing = opposing.drain_filter(|(_, _, size)| size.0 > dec!(0)).collect::<Vec<_>>();
    if order_size.0 > dec!(0) {
        return (Some(order_size), matched_opposing)
    }
    (None, matched_opposing)
}

impl Handler<messages::RegisterTrader> for MarketActor {
    type Result = ();

    fn handle(&mut self, msg: messages::RegisterTrader, _ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Registering for market updates");

        let update_msg = self.tick_data_refresh_msg();
        msg.1.do_send(update_msg);
        let state = InternalTraderState {
            recp: msg.1,
            open_orders: HashMap::new(),
            matched_orders: HashMap::new(),
        };
        self.subscribers.insert(msg.0, state);
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

        Self { order_book, subscribers: HashMap::new(), bots: Vec::new() }
    }

    pub fn update_listeners(&self, msg: messages::TickDataUpdate, _ctx: &mut Context<Self>) {
        for (_, trader) in self.subscribers.iter() {
            trader.recp.do_send(msg.clone());
        }
    }

    fn tick_data_refresh_msg(&mut self) -> messages::TickDataUpdate {
        let mut tick_data =
            self.order_book.values_mut().map(compress_order_book_range).collect::<Vec<_>>();
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
