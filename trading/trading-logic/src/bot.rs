use std::time::Duration;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use rand::Rng;
use rust_decimal_macros::dec;
use trading_types::common::{Order, Side, Size, Tick, TraderId};

use crate::market::messages::{PlaceOrder, TickDataUpdate};
use crate::market::MarketActor;

pub struct BotActor {
    trader_id: TraderId,
    market: Addr<MarketActor>,
    next_placement_in: Duration,
    next_placement_size: Size,
    next_placement_side: Side,
    next_placement_tick: Tick,
    random: rand::rngs::ThreadRng,
}

impl BotActor {
    pub fn new(market: Addr<MarketActor>, trader_id: TraderId) -> Self {
        let random = rand::thread_rng();
        let mut instance = Self {
            trader_id,
            market,
            next_placement_side: Side::Back,
            random,
            next_placement_in: Duration::from_millis(300),
            next_placement_size: Size(dec!(2.0)),
            next_placement_tick: Tick(dec!(1.51)),
        };
        instance.roll_side();
        instance
    }

    fn roll_side(&mut self) {
        self.next_placement_side = if self.random.gen_bool(0.5) { Side::Back } else { Side::Lay };
    }
}

impl Actor for BotActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.market.do_send(crate::market::messages::RegisterTrader(
            self.trader_id.clone(),
            ctx.address().recipient(),
        ));

        ctx.notify_later(PlaceNextBet, self.next_placement_in);
    }
}

impl Handler<TickDataUpdate> for BotActor {
    type Result = ();

    fn handle(&mut self, msg: TickDataUpdate, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            TickDataUpdate::SingleUpdate(msg) => {
                let next_placement_size = self.random.gen_range(2..10);
                self.next_placement_size = Size(rust_decimal::Decimal::new(next_placement_size, 0));

                let next_placement_tick_diff = self.random.gen_range(-7..7);
                let next_placement_tick = msg
                    .tick
                    .0
                    .checked_add(rust_decimal::Decimal::new(next_placement_tick_diff, 2))
                    .unwrap();

                if next_placement_tick > dec!(2.00) {
                    self.next_placement_tick = Tick(dec!(2.00));
                } else if next_placement_tick < dec!(1.01) {
                    self.next_placement_tick = Tick(dec!(1.01));
                } else {
                    self.next_placement_tick = Tick(next_placement_tick);
                }

                self.roll_side();
            }
            TickDataUpdate::SetRefresh(_msg) => (),
        };
    }
}

impl Handler<PlaceNextBet> for BotActor {
    type Result = ();

    fn handle(&mut self, _msg: PlaceNextBet, ctx: &mut Context<Self>) -> Self::Result {
        let msg = PlaceOrder {
            trader: self.trader_id.clone(),
            order: Order {
                tick: self.next_placement_tick.clone(),
                size: self.next_placement_size.clone(),
                side: self.next_placement_side.clone(),
            },
        };

        self.next_placement_in =
            Duration::from_millis(self.random.gen_range(100..200) + self.random.gen_range(0..1000));
        ctx.notify_later(PlaceNextBet, self.next_placement_in);

        self.market.do_send(msg);
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
struct PlaceNextBet;

#[test]
fn testss() {
    let mut random = rand::thread_rng();
    // let current_tick = Tick(dec!(1.51));
    let next_placement_tick_diff = random.gen_range(-2..2);
    // let next_placement_tick = current_tick
    //                 .0
    //                 .saturating_add(rust_decimal::Decimal::new(next_placement_tick_diff, 2));

    // panic!("{}", next_placement_tick);

    let val = rust_decimal::Decimal::new(next_placement_tick_diff, 2);
    panic!("test {}", val);
}
