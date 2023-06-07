use std::rc::Rc;

use actix::{
    Actor, ActorContext, ActorFutureExt, Addr, Context, ContextFutureSpawner, StreamHandler,
    WrapFuture,
};
use axum::extract::ws::{self, CloseFrame, Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use trading_logic::market::MarketActor;

use crate::state::WebAppState;

pub async fn handler(
    ws: WebSocketUpgrade,
    Path(market_id): Path<String>,
    State(state): State<WebAppState>,
) -> impl IntoResponse {
    let ws = ws.on_upgrade(move |ws| handle_connection(state, ws, market_id));
    ws
}


async fn handle_connection(
    state: WebAppState,
    websocket: axum::extract::ws::WebSocket,
    market_id: String,
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

    fn started(&mut self, ctx: &mut Self::Context) {
    }
}

impl StreamHandler<Result<WsMsg, axum::Error>> for WsActor {
    fn handle(&mut self, item: Result<WsMsg, axum::Error>, ctx: &mut Context<WsActor>) {
        // TODO forward the message to either the market or respond back immediately
    }
}
