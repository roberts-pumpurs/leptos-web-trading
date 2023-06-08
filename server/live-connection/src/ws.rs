use std::rc::Rc;

use actix::{
    Actor, ActorContext, ActorFutureExt, Addr, Context, ContextFutureSpawner, StreamHandler,
    WrapFuture,
};
use axum::extract::ws::{self, WebSocket};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use state::WebAppState;
use tokio::sync::Mutex;
use trading_logic::market::MarketActor;

pub async fn handle_connection(
    state: WebAppState,
    websocket: axum::extract::ws::WebSocket,
    market_id: u32,
) {
    let (ws_sender, ws_receiver) = websocket.split();
    if let Some(market) = state.markets().get(&market_id) {
        let market = market.clone();
        let _actor = WsActor::start_in_arbiter(state.arb(), move |ctx| {
            let stream = ws_receiver.map(|x| x.map(WsMsg));
            WsActor::add_stream(stream, ctx);
            WsActor { agent: nanoid::nanoid!(), sender: Rc::new(Mutex::new(ws_sender)), market }
        });
    }
}

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
struct WsMsg(pub ws::Message);

struct WsActor {
    agent: String,
    market: Addr<MarketActor>,
    sender: Rc<Mutex<SplitSink<WebSocket, ws::Message>>>,
}

impl WsActor {
    #[tracing::instrument(skip(self, ctx), fields(agent = %self.agent))]
    fn send(&self, msg: ws::Message, ctx: &mut Context<Self>) {
        let sender = self.sender.clone();
        async move { sender.lock().await.send(msg).await }
            .into_actor(self)
            .map(|a, b, c| {
                if a.is_err() {
                    c.stop();
                }
                async {}.into_actor(b).spawn(c);
            })
            .spawn(ctx);
    }
}

impl Actor for WsActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!(agent = self.agent, "ws actor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::warn!(agent = self.agent, "ws actor stopped");
    }
}

impl StreamHandler<Result<WsMsg, axum::Error>> for WsActor {
    fn handle(&mut self, _item: Result<WsMsg, axum::Error>, _ctx: &mut Context<WsActor>) {
        // TODO forward the message to either the market or respond back immediately
    }
}
