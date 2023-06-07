use std::collections::HashMap;

use actix::*;
use trading_logic::market::MarketActor;

#[derive(Debug, Clone)]
pub struct WebAppState {
    arb: ArbiterHandle,
    markets: HashMap<String, Addr<MarketActor>>,
}

impl WebAppState {
    pub fn new(arb: ArbiterHandle) -> Self {
        let mut markets = HashMap::new();
        for market in &["1", "2", "3"] {
            let market_actor = Self::spawn_market(&arb);
            markets.insert(market.to_string(), market_actor);
        }
        Self { arb, markets }
    }

    pub fn arb(&self) -> &ArbiterHandle {
        &self.arb
    }

    fn spawn_market(arb: &ArbiterHandle) -> Addr<MarketActor> {
        MarketActor::start_in_arbiter(arb, move |_ctx| MarketActor::new())
    }

    pub fn markets(&self) -> &HashMap<String, Addr<MarketActor>> {
        &self.markets
    }
}
