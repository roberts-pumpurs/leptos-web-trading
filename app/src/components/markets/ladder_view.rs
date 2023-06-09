use futures::{FutureExt, SinkExt, StreamExt};
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
    let id = create_memo::<u32>(cx, move |_| {
        params.with(|params| {
            params.get("id").cloned().map(|x| x.parse::<u32>().unwrap_or(1)).unwrap_or(1)
        })
    });

    view! { cx, <LadderViewInternal id=id/> }
}

#[derive(Debug, Clone)]
struct SenderWrapper {
    sender: futures::channel::mpsc::Sender<Option<TraderMessage>>,
    url: String,
}

impl PartialEq for SenderWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

#[component]
fn LadderViewInternal(cx: Scope, id: Memo<u32>) -> impl IntoView {
    let derived_ws_url = create_memo::<String>(cx, move |_| derive_ws_url(id()));
    let ws_client_sender = create_memo::<Option<SenderWrapper>>(cx, move |prev| {
        // Stop the previous ws connection
        match prev {
            Some(Some(x)) => {
                let mut x = x.clone();
                spawn_local(async move {
                    let _ = x.sender.send(None).await;
                });
            }
            _ => (),
        };

        if let Ok(ws_client) = WebSocket::open(&derived_ws_url()) {
            let (to_ws_sender, mut to_ws_recv) =
                futures::channel::mpsc::channel::<Option<TraderMessage>>(100);
            {
                let to_ws_sender = to_ws_sender.clone();
                spawn_local(async move {
                    let mut ws_client = ws_client.fuse();
                    let interval = {
                        let to_ws_sender = to_ws_sender.clone();
                        gloo_timers::callback::Interval::new(3_000, move || {
                            let mut to_ws_sender = to_ws_sender.clone();
                            spawn_local(async move {
                                let ms = current_time_ms();
                                let msg = TraderMessage::TraderTime { ms };
                                let _ = to_ws_sender.send(Some(msg)).await;
                            });
                        })
                    };

                    let mut to_ws_sender = to_ws_sender.clone();
                    loop {
                        futures::select! {
                            msg = ws_client.next() => {
                                match msg {
                                    Some(Ok(Message::Bytes(msg))) => {
                                        let Ok(value) = ciborium::from_reader::<ServerMessage, _>(&msg[..]) else {
                                            continue;
                                        };
                                        log!("received msg from server {:?}", value);
                                        match value {
                                            ServerMessage::TraderTimeAck => {
                                                let ms = current_time_ms();
                                                let msg = TraderMessage::TraderTimeAck { ms };
                                                let _ = to_ws_sender.send(Some(msg)).await;
                                            }
                                            ServerMessage::ConnectionInfo(_) => (),
                                            ServerMessage::TickSetWhole(_) => (),
                                            ServerMessage::TickUpdate(_) => (),
                                            ServerMessage::OrderAccepted(_) => (),
                                            ServerMessage::OrderRejected(_, _) => (),
                                        }
                                    }
                                    _ => break, // don't act on text msgs
                                }
                            }
                            msg = to_ws_recv.next() => {
                                match msg {
                                    Some(Some(msg)) => {
                                        log!("Sending msg to server {:?}", msg);
                                        let mut writer = Vec::new();
                                        if let Ok(_) = ciborium::into_writer(&msg, &mut writer) {
                                            let msg = Message::Bytes(writer);
                                            if ws_client.send(msg).await.is_err() {
                                                break
                                            }
                                        }
                                    }
                                    _ => break,
                                }
                            }
                        }
                    }
                    interval.cancel();
                    let _ = ws_client.close().await;
                    log!("WS client closed");
                });
            }
            return Some(SenderWrapper { sender: to_ws_sender, url: derived_ws_url() })
        }
        None
    });

    create_effect(cx, move |_| {
        log!("ws_client_sender {:?}", ws_client_sender());
    });

    on_cleanup(cx, move || {
        log!("Running cleanup");
        match ws_client_sender() {
            Some(mut ws_client_sender) => {
                spawn_local(async move {
                    let _ = ws_client_sender.sender.send(None).await;
                });
            }
            None => (),
        }
    });

    view! { cx,
        <div class="HomeView">
            <LadderTable/>
        </div>
    }
}

fn current_time_ms() -> u64 {
    let now = js_sys::Date::new_0();

    now.get_time() as u64
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
