use std::collections::HashMap;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, Recipient};
use nanoid::nanoid;
use rust_decimal_macros::dec;
use trading_types::common::{Order, RequestId, Size, Tick, TraderId};
use trading_types::from_server::TickData;

use self::messages::PlaceOrder;
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
    pub struct RegisterTrader(
        pub TraderId,
        pub Recipient<TickDataUpdate>,
        pub Recipient<OrderStateUpdate>,
    );

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub struct OrderStateUpdate {
        pub open_orders: HashMap<Tick, Order>,
        pub matched_orders: HashMap<Tick, Order>,
    }

    #[derive(Message, Debug, Clone)]
    #[rtype(result = "()")]
    pub enum TickDataUpdate {
        SetRefresh(Vec<TickData>),
        SingleUpdate(TickData),
        NewLatestMatch(TickData),
    }
}

pub struct MarketActor {
    order_book: HashMap<Tick, OrderBookRange>,
    traders: HashMap<TraderId, InternalTraderState>,
    bots: Vec<Addr<BotActor>>,
}

struct InternalTraderState {
    recp_tick_update: Recipient<messages::TickDataUpdate>,
    recp_order_update: Recipient<messages::OrderStateUpdate>,
    open_orders: HashMap<Tick, Order>,
    matched_orders: HashMap<Tick, Order>,
}

impl Actor for MarketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Reset the game every minute
        ctx.run_interval(std::time::Duration::from_secs(60), |act, _ctx| {
            for (_key, val) in act.order_book.iter_mut() {
                val.clear();
            }
            for (_key, val) in act.traders.iter_mut() {
                val.clear();
            }

            let new_balance = Tick(dec!(1.50));
            if let Some(obr) = act.order_book.get_mut(&new_balance) {
                let new_balance = compress_order_book_range(obr);
                act.update_listeners(messages::TickDataUpdate::NewLatestMatch(new_balance));
            }
            let update_msg = act.tick_data_refresh_msg();
            act.update_listeners(update_msg);

            for (_, trader) in act.traders.iter() {
                let update_msg = messages::OrderStateUpdate {
                    open_orders: trader.open_orders.clone(),
                    matched_orders: trader.matched_orders.clone(),
                };
                trader.recp_order_update.do_send(update_msg);
            }
        });
    }
}

impl Handler<messages::PlaceOrder> for MarketActor {
    type Result = ();

    fn handle(&mut self, msg: messages::PlaceOrder, _ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Received order");
        let Some(trader) = self.traders.get_mut(&msg.trader) else {
            return;
        };

        let tick = msg.order.tick;
        if let Some(obr) = self.order_book.get_mut(&msg.order.tick) {
            let affected_traders = match msg.order.side {
                trading_types::common::Side::Back => Self::match_orders(
                    msg,
                    &mut obr.open_lays,
                    &mut obr.total_matched,
                    &mut obr.open_backs,
                    trader,
                ),
                trading_types::common::Side::Lay => Self::match_orders(
                    msg,
                    &mut obr.open_backs,
                    &mut obr.total_matched,
                    &mut obr.open_lays,
                    trader,
                ),
            };
            // Send tick update to all listeners
            let tick_data = compress_order_book_range(obr);
            if !affected_traders.is_empty() {
                self.update_listeners(messages::TickDataUpdate::NewLatestMatch(tick_data.clone()));
            }
            self.update_listeners(messages::TickDataUpdate::SingleUpdate(tick_data));

            // Send individual order updates to affected traders
            for (trader_id, new_size) in affected_traders {
                let Some(trader) = self.traders.get_mut(&trader_id) else {
                    continue;
                };

                if new_size.0 == dec!(0) {
                    trader.open_orders.remove(&tick);
                } else if let Some(order) = trader.open_orders.get_mut(&tick) {
                    order.size = new_size;
                }

                let update_msg = messages::OrderStateUpdate {
                    open_orders: trader.open_orders.clone(),
                    matched_orders: trader.matched_orders.clone(),
                };
                trader.recp_order_update.do_send(update_msg);
            }
        }
    }
}

impl MarketActor {
    fn match_orders(
        order: PlaceOrder,
        opposing_orders: &mut Vec<(TraderId, RequestId, Size)>,
        matched_aggregate: &mut Size,
        aligned_orders: &mut Vec<(TraderId, RequestId, Size)>,
        trader: &mut InternalTraderState,
    ) -> Vec<(TraderId, Size)> {
        trader.open_orders.insert(order.order.tick, order.order.clone());

        let mut affected_traders = vec![];
        let mut leftover_amount = order.order.size;
        let mut matched_amount = Size(dec!(0));

        for (opposing_trader_id, _opposing_req_id, opposing_order_size) in
            opposing_orders.iter_mut()
        {
            match leftover_amount.cmp(opposing_order_size) {
                std::cmp::Ordering::Less => {
                    opposing_order_size.0 -= leftover_amount.0;
                    matched_amount.0 += leftover_amount.0;
                    leftover_amount.0 = dec!(0);
                }
                std::cmp::Ordering::Equal => {
                    matched_amount.0 += leftover_amount.0;
                    opposing_order_size.0 = dec!(0);
                    leftover_amount.0 = dec!(0);
                }
                std::cmp::Ordering::Greater => {
                    leftover_amount.0 -= opposing_order_size.0;
                    matched_amount.0 += opposing_order_size.0;
                    opposing_order_size.0 = dec!(0);
                }
            }
            affected_traders.push((opposing_trader_id.clone(), *opposing_order_size));

            if order.order.size.0 == dec!(0) {
                break
            }
        }
        matched_aggregate.0 += matched_amount.0;

        if leftover_amount.0 > dec!(0) {
            aligned_orders.push((order.trader.clone(), order.request_id.clone(), leftover_amount));
            if let Some(trader) = trader.open_orders.get_mut(&order.order.tick) {
                trader.size = leftover_amount;
            } else {
                trader.open_orders.insert(
                    order.order.tick,
                    Order { size: leftover_amount, ..order.order.clone() },
                );
            }
        }
        if matched_amount.0 > dec!(0) {
            if let Some(trader) = trader.matched_orders.get_mut(&order.order.tick) {
                trader.size.0 += matched_amount.0;
            } else {
                trader.matched_orders.insert(
                    order.order.tick,
                    Order { size: matched_amount, ..order.order.clone() },
                );
            }
        }

        let update_msg = messages::OrderStateUpdate {
            open_orders: trader.open_orders.clone(),
            matched_orders: trader.matched_orders.clone(),
        };
        trader.recp_order_update.do_send(update_msg);

        *opposing_orders =
            opposing_orders.drain_filter(|(_, _, size)| size.0 > dec!(0)).collect::<Vec<_>>();

        affected_traders
    }
}

impl Handler<messages::RegisterTrader> for MarketActor {
    type Result = ();

    fn handle(&mut self, msg: messages::RegisterTrader, _ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "Registering for market updates");

        let update_msg = self.tick_data_refresh_msg();
        msg.1.do_send(update_msg);
        let state = InternalTraderState {
            recp_tick_update: msg.1,
            recp_order_update: msg.2,
            open_orders: HashMap::new(),
            matched_orders: HashMap::new(),
        };

        let update_msg = messages::OrderStateUpdate {
            open_orders: state.open_orders.clone(),
            matched_orders: state.matched_orders.clone(),
        };
        state.recp_order_update.do_send(update_msg);
        self.traders.insert(msg.0, state);
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
    let open_backs = value.open_backs.iter().map(|x| &x.2).fold(Size(dec!(0)), |acc, i| acc + i);
    let open_lays = value.open_lays.iter().map(|x| &x.2).fold(Size(dec!(0)), |acc, i| acc + i);
    let total_matched = value.total_matched;
    TickData {
        tick: value.tick,
        total_matched,
        available_backs: open_backs,
        available_lays: open_lays,
    }
}

impl MarketActor {
    pub fn new() -> Self {
        let mut order_book = HashMap::new();
        for tick in Tick::all() {
            order_book.insert(tick, OrderBookRange::new(tick));
        }

        Self { order_book, traders: HashMap::new(), bots: Vec::new() }
    }

    pub fn update_listeners(&self, msg: messages::TickDataUpdate) {
        for (_, trader) in self.traders.iter() {
            trader.recp_tick_update.do_send(msg.clone());
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
    open_backs: Vec<(TraderId, RequestId, Size)>,
    open_lays: Vec<(TraderId, RequestId, Size)>,
    total_matched: Size,
    tick: Tick,
}

impl OrderBookRange {
    fn new(tick: Tick) -> Self {
        Self { open_backs: Vec::new(), open_lays: Vec::new(), tick, total_matched: Size(dec!(0)) }
    }

    fn clear(&mut self) {
        self.open_backs.clear();
        self.open_lays.clear();
        self.total_matched = Size(dec!(0));
    }
}
impl InternalTraderState {
    fn clear(&mut self) {
        self.matched_orders.clear();
        self.open_orders.clear();
    }
}
