use std::collections::HashMap;

use actix::*;
use axum::extract::FromRef;
use leptos::LeptosOptions;
use trading_logic::market::messages::SpawnBot;
use trading_logic::market::MarketActor;

use crate::get_markets;

#[derive(FromRef, Debug, Clone)]
pub struct WebAppState {
    leptos_options: LeptosOptions,
    arb: ArbiterHandle,
    markets: HashMap<u32, Addr<MarketActor>>,
}

impl WebAppState {
    pub fn new(arb: ArbiterHandle, leptos_options: LeptosOptions) -> Self {
        let mut markets = HashMap::new();
        for market in get_markets() {
            let market_actor = Self::spawn_market(&arb);
            for _ in 0..market.bots {
                market_actor.do_send(SpawnBot);
            }
            markets.insert(market.id, market_actor);
        }
        Self { arb, markets, leptos_options }
    }

    pub fn arb(&self) -> &ArbiterHandle {
        &self.arb
    }

    fn spawn_market(arb: &ArbiterHandle) -> Addr<MarketActor> {
        MarketActor::start_in_arbiter(arb, move |_ctx| MarketActor::new())
    }

    pub fn markets(&self) -> &HashMap<u32, Addr<MarketActor>> {
        &self.markets
    }
}
