use std::rc::Rc;

use actix::{
    Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner,
    Handler, StreamHandler, WrapFuture,
};
use anyhow::anyhow;
use axum::extract::ws::{self, WebSocket};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use rust_decimal_macros::dec;
use state::WebAppState;
use tokio::sync::Mutex;
use trading_logic::market::messages::{RegisterForUpdates, TickDataUpdate};
use trading_logic::market::MarketActor;
use trading_types::common::{Size, TraderId};
use trading_types::from_server::{Latency, ServerMessage, TraderInfo};
use trading_types::from_trader::TraderMessage;

pub async fn handle_connection(
    state: WebAppState,
    websocket: axum::extract::ws::WebSocket,
    market_id: u32,
) {
    let (ws_sender, ws_receiver) = websocket.split();
    if let Some(market) = state.markets().get(&market_id) {
        let market = market.clone();
        let _actor = WsActor::start_in_arbiter(state.arb(), move |ctx| {
            let stream = ws_receiver.map(|x| {
                x.map(|x| {
                    tracing::debug!("received message {:?}", x);
                    let res = match x {
                        ws::Message::Binary(x) => {
                            let value = ciborium::from_reader::<TraderMessage, _>(&x[..]);
                            match value {
                                Ok(value) => Ok(value),
                                Err(_) => Err(anyhow!("invalid message")),
                            }
                        }
                        _ => Err(anyhow!("invalid message")),
                    };
                    WsMsg(res)
                })
                .map_err(|_| anyhow!("Axum WS error"))
            });
            WsActor::add_stream(stream, ctx);
            WsActor {
                trader_id: TraderId(nanoid::nanoid!()),
                sender: Rc::new(Mutex::new(ws_sender)),
                market,
                last_trader_time_ms: chrono::Utc::now().timestamp_millis() as u64,
                last_trader_info: TraderInfo {
                    exposure: Size(dec!(0.0)),
                    balance: Size(dec!(10000.0)),
                    orders: vec![],
                },
            }
        });
    }
}

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
struct WsMsg(pub Result<TraderMessage, anyhow::Error>);

struct WsActor {
    trader_id: TraderId,
    market: Addr<MarketActor>,
    sender: Rc<Mutex<SplitSink<WebSocket, ws::Message>>>,
    last_trader_time_ms: u64,
    last_trader_info: TraderInfo,
}

impl WsActor {
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

    fn send_server_message(&self, msg: ServerMessage, ctx: &mut Context<Self>) {
        let mut writer = Vec::new();
        if let Ok(_) = ciborium::into_writer(&msg, &mut writer) {
            let msg = ws::Message::Binary(writer);
            self.send(msg, ctx);
        }
    }
}

impl Actor for WsActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!(agent =? self.trader_id, "ws actor started");
        let recp = ctx.address().recipient::<TickDataUpdate>();
        self.market.do_send(RegisterForUpdates(recp));
        self.send_server_message(ServerMessage::TraderTimeAck, ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::warn!(agent =? self.trader_id, "ws actor stopped");
    }
}

impl StreamHandler<Result<WsMsg, anyhow::Error>> for WsActor {
    fn handle(&mut self, item: Result<WsMsg, anyhow::Error>, ctx: &mut Context<WsActor>) {
        // TODO forward the message to either the market or respond back immediately
        if let Ok(WsMsg(Ok(msg))) = item {
            match msg {
                TraderMessage::PlaceOrder(_req_id, order) => {
                    self.market.do_send(trading_logic::market::messages::PlaceOrder(
                        self.trader_id.clone(),
                        order,
                    ));
                }
                TraderMessage::TraderTime {ms: time} => {
                    self.last_trader_time_ms = time;
                    self.send_server_message(ServerMessage::TraderTimeAck, ctx)
                }
                TraderMessage::TraderTimeAck {ms: time} => {
                    let latency = time.abs_diff(self.last_trader_time_ms);
                    let latency = Latency { ms: latency };
                    self.send_server_message(ServerMessage::ConnectionInfo(latency), ctx)
                }
            };
        }
    }
}

impl Handler<TickDataUpdate> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: TickDataUpdate, ctx: &mut Context<Self>) -> Self::Result {
        tracing::info!(msg = ?msg, "TickDataUpdate");
        let msg = match msg {
            TickDataUpdate::SetRefresh(msg) => ServerMessage::TickSetWhole(msg),
            TickDataUpdate::SingleUpdate(msg) => ServerMessage::TickUpdate(msg),
        };
        self.send_server_message(msg, ctx);
    }
}
