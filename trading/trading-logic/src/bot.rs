use std::time::Duration;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use rand::Rng;
use rust_decimal_macros::dec;
use trading_types::common::{Order, RequestId, Side, Size, Tick, TraderId};

use crate::market::messages::{OrderStateUpdate, PlaceOrder, TickDataUpdate};
use crate::market::MarketActor;

pub struct BotActor {
    trader_id: TraderId,
    market: Addr<MarketActor>,
    next_placement_order: Order,
    random: rand::rngs::ThreadRng,
    spawn_handle: Option<actix::SpawnHandle>,
}

impl BotActor {
    pub fn new(market: Addr<MarketActor>, trader_id: TraderId) -> Self {
        let random = rand::thread_rng();
        let mut instance = Self {
            trader_id,
            next_placement_order: Order {
                size: Size(dec!(2.0)),
                side: Side::Back,
                tick: Tick(dec!(1.50)),
            },
            spawn_handle: None,
            market,
            random,
        };
        instance.roll_new_order(instance.next_placement_order.tick);
        instance
    }
}

impl Actor for BotActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.market.do_send(crate::market::messages::RegisterTrader(
            self.trader_id.clone(),
            ctx.address().recipient(),
            ctx.address().recipient(),
        ));

        let next_placement_in = Duration::from_millis(self.random.gen_range(500..2000));
        ctx.notify_later(PlaceNextBet, next_placement_in);
    }
}

impl Handler<TickDataUpdate> for BotActor {
    type Result = ();

    fn handle(&mut self, msg: TickDataUpdate, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            TickDataUpdate::NewLatestMatch(msg) => {
                if self.random.gen_bool(0.5) {
                    self.roll_new_order(msg.tick);
                }
            }
            TickDataUpdate::SetRefresh(_msg) => {
                if let Some(spawn_handle) = self.spawn_handle.take() {
                    ctx.cancel_future(spawn_handle);
                    self.roll_new_order(Tick(dec!(1.50)));
                }
            }
            TickDataUpdate::SingleUpdate(msg) => {
                if self.random.gen_bool(0.05) {
                    let (side, size) = if msg.available_backs.0 > msg.available_lays.0 {
                        let half_size = msg.available_lays.0 / dec!(2.0);
                        (Side::Back, Size(half_size))
                    } else {
                        let half_size = msg.available_backs.0 / dec!(2.0);
                        (Side::Lay, Size(half_size))
                    };

                    let msg = PlaceOrder {
                        request_id: RequestId(nanoid::nanoid!()),
                        trader: self.trader_id.clone(),
                        order: Order { side, size, tick: msg.tick },
                    };
                    self.market.do_send(msg);
                }
            }
        };
    }
}
impl Handler<OrderStateUpdate> for BotActor {
    type Result = ();

    fn handle(&mut self, _msg: OrderStateUpdate, _ctx: &mut Context<Self>) -> Self::Result {
        // noop
    }
}

impl Handler<PlaceNextBet> for BotActor {
    type Result = ();

    fn handle(&mut self, _msg: PlaceNextBet, ctx: &mut Context<Self>) -> Self::Result {
        let msg = PlaceOrder {
            request_id: RequestId(nanoid::nanoid!()),
            trader: self.trader_id.clone(),
            order: self.next_placement_order.clone(),
        };
        self.market.do_send(msg);
        self.roll_new_order(self.next_placement_order.tick);

        // Schedule next placement
        let next_placement_in = Duration::from_millis(self.random.gen_range(500..2000));
        self.spawn_handle = Some(ctx.notify_later(PlaceNextBet, next_placement_in));
    }
}

impl BotActor {
    fn roll_new_order(&mut self, prev_balance: Tick) {
        let next_placement_side = if self.random.gen_bool(0.5) { Side::Back } else { Side::Lay };
        let next_placement_size =
            Size(rust_decimal::Decimal::new(self.random.gen_range(2..300), 0));
        let next_placement_tick = self.gen_new_tick(next_placement_side, prev_balance);

        self.next_placement_order = Order {
            tick: next_placement_tick,
            size: next_placement_size,
            side: next_placement_side,
        };
    }

    fn gen_new_tick(&mut self, next_placement_side: Side, prev_balance: Tick) -> Tick {
        let next_placement_tick_diff = {
            match next_placement_side {
                Side::Back => self.random.gen_range(-2..=0),
                Side::Lay => self.random.gen_range(0..=2),
            }
        };
        let next_placement_tick = prev_balance
            .0
            .checked_add(rust_decimal::Decimal::new(next_placement_tick_diff, 2))
            .unwrap();
        Tick(next_placement_tick)
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
struct PlaceNextBet;
