use std::sync::{Arc, RwLock};

use chrono::{DateTime, Utc};
use futures::channel::oneshot;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use gloo_timers;
use leptos::*;
use leptos_router::*;
use trading_types::from_server::ServerMessage;
use trading_types::from_trader::TraderMessage;

#[component]
pub fn LadderView(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || {
        params.with(|params| {
            params.get("id").cloned().map(|x| x.parse::<u32>().unwrap_or(1)).unwrap_or(1)
        })
    };
    let derived_ws_url = create_memo::<String>(cx, move |_| derive_ws_url(id()));
    create_effect(cx, move |_| {
        if let Ok(ws_client) = WebSocket::open(&derived_ws_url()) {
            let (to_ws_sender, mut to_ws_recv) =
                futures::channel::mpsc::channel::<TraderMessage>(100);
            let (mut sender, mut recv) = ws_client.split();
            let to_ws_sender = Arc::new(RwLock::new(to_ws_sender));
            // Server -> Client
            {
                let to_ws_sender = to_ws_sender.clone();
                spawn_local(async move {
                    while let Some(Ok(msg)) = recv.next().await {
                        match msg {
                            Message::Bytes(msg) => {
                                let Ok(value) = ciborium::from_reader::<ServerMessage, _>(&msg[..]) else {
                                continue;
                            };
                                log!("received msg from server {:?}", value);
                                match value {
                                    ServerMessage::TraderTimeAck => {
                                        let mut sender = to_ws_sender.write().unwrap();

                                        let now = js_sys::Date::new_0();
                                        let ms = now.get_time() as u64;
                                        let msg = TraderMessage::TraderTimeAck { ms };
                                        let _ = sender.send(msg).await;
                                    }
                                    ServerMessage::ConnectionInfo(_) => (),
                                    ServerMessage::TickSetWhole(_) => (),
                                    ServerMessage::TickUpdate(_) => (),
                                    ServerMessage::OrderAccepted(_) => (),
                                    ServerMessage::OrderRejected(_, _) => (),
                                }
                            }
                            _ => (), // don't act on text msgs
                        }
                    }
                });
            }

            // Client -> Server
            spawn_local(async move {
                while let Some(msg) = to_ws_recv.next().await {
                    let mut writer = Vec::new();
                    if let Ok(_) = ciborium::into_writer(&msg, &mut writer) {
                        let msg = Message::Bytes(writer);
                        if sender.send(msg).await.is_err() {
                            // TODO reunite with the sender
                        }
                    }
                }
            });

            // Every 5 seconds, send a ping to the server
            {
                let to_ws_sender = to_ws_sender.clone();
                spawn_local(async move {
                    use gloo_timers::callback::Interval;
                    let _ = Interval::new(2_000, move || {
                        let to_ws_sender = to_ws_sender.clone();
                        spawn_local(async move {
                            log!("sending ping to server");
                            let mut sender = to_ws_sender.write().unwrap();
                            let now = js_sys::Date::new_0();
                            let ms = now.get_time() as u64;
                            let msg = TraderMessage::TraderTime { ms };
                            let _ = sender.send(msg).await;
                        })
                    })
                    .forget();
                });
            }

            // TODO create cleanup that makes sure we destroy all websockets
        } else {
            warn!("failed to connect to websocket");
        }
    });

    view! { cx,
        <div class="HomeView">
            <LadderTable/>
        </div>
    }
}

fn derive_ws_url(id: u32) -> String {
    let host = window().location().host().unwrap_or("127.0.0.1:3000".to_string());
    let protocol = {
        if window().location().protocol().unwrap_or("http".to_string()).contains("https") {
            "wss"
        } else {
            "ws"
        }
    };
    format!("{}://{}/ws/{}", protocol, host, id)
}

#[component]
fn LadderTable(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="px-4 sm:px-6 lg:px-8">
            <div class="mt-8 flow-root">
                <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
                        <table class="table-fixed min-w-full divide-y divide-gray-300">
                            <thead>
                                <tr class="divide-x divide-gray-200">
                                    <th
                                        scope="col"
                                        class="w-1/6 whitespace-nowrap py-3.5 pl-4 pr-3 text-center text-sm font-semibold text-gray-900 sm:pl-0"
                                    >
                                        "Lapse"
                                    </th>
                                    <th
                                        scope="col"
                                        class="w-1/6 whitespace-nowrap px-2 py-3.5 text-center text-sm font-semibold  bg-gray-200 text-gray-500"
                                    >
                                        "Back"
                                    </th>
                                    <th
                                        scope="col"
                                        class="w-1/6 whitespace-nowrap px-2 py-3.5 text-center text-sm font-semibold  bg-gray-200 text-gray-500"
                                    >
                                        "Odds"
                                    </th>
                                    <th
                                        scope="col"
                                        class="w-1/6 whitespace-nowrap px-2 py-3.5 text-center text-sm font-semibold  bg-gray-200 text-gray-500"
                                    >
                                        "Lay"
                                    </th>
                                    <th
                                        scope="col"
                                        class="w-1/6 whitespace-nowrap px-2 py-3.5 text-center text-sm font-semibold text-gray-900"
                                    >
                                        "Lapse"
                                    </th>
                                    <th
                                        scope="col"
                                        class="w-1/6 whitespace-nowrap px-2 py-3.5 text-center text-sm font-semibold bg-gray-200 text-gray-500"
                                    >
                                        "Liquidity"
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="text-center divide-y divide-gray-200 bg-white ">
                                <tr class="divide-x divide-gray-200">
                                    <td class="w-1/6 whitespace-nowrap text-sm text-gray-500 sm:pl-0">
                                        <input type="number" class="w-full"/>
                                    </td>
                                    <td class="w-1/6 whitespace-nowrap text-sm bg-blue-200 text-blue-950">
                                        "120e"
                                    </td>
                                    <td class="w-1/6 text-center whitespace-nowrap text-sm text-white bg-slate-500">
                                        "1.01"
                                    </td>
                                    <td class="w-1/6 whitespace-nowrap text-sm bg-red-200 text-red-950">
                                        "120e"
                                    </td>
                                    <td class="w-1/6 whitespace-nowrap text-sm text-gray-500">
                                        <input type="number" class="w-full"/>
                                    </td>
                                    <td class="w-1/6 text-center whitespace-nowrap text-sm text-gray-500 bg-slate-200">
                                        "500e"
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}
