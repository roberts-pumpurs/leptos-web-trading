use futures::{FutureExt, SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use gloo_timers;
use leptos::*;
use leptos_router::*;
use rust_decimal_macros::dec;
use trading_types::common::Tick;
use trading_types::from_server::{Latency, ServerMessage, TickData};
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
    let (latency, set_latency) = create_signal::<Option<Latency>>(cx, None);
    let (ladder, set_ladder) = create_signal::<Option<(Vec<TickData>, Tick)>>(cx, None);
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
                futures::channel::mpsc::channel::<Option<TraderMessage>>(5);
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
                                            ServerMessage::ConnectionInfo(latency) => {
                                                set_latency(Some(latency));
                                            },
                                            ServerMessage::TickSetWhole(set) => {
                                                set_ladder(Some((set, Tick(dec!(1.51)))));
                                            },
                                            ServerMessage::TickUpdate(new_value) => {
                                                set_ladder.update(|ladder| {
                                                    if let Some((ladder, last_traded)) = ladder {

                                                        *last_traded = new_value.tick.clone();
                                                        if let Some(val) = ladder.iter_mut().find(|x| x.tick == new_value.tick) {
                                                            *val = new_value;
                                                        }
                                                    }
                                                });
                                            },
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
                    set_latency(None);
                    set_ladder(None);
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
            <StatsComponent latency=latency/>
            <LadderTable ladder=ladder/>
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
fn StatsComponent(cx: Scope, latency: ReadSignal<Option<Latency>>) -> impl IntoView {
    view! { cx,
        <div>
            <h3 class="text-base font-semibold leading-6 text-gray-900">"Stats"</h3>
            <dl class="mt-5 grid grid-cols-1 divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow md:grid-cols-3 md:divide-x md:divide-y-0">
                <div class="px-4 py-5 sm:p-6">
                    <dt class="text-base font-normal text-gray-900">"WS Latency"</dt>
                    <dd class="mt-1 flex items-baseline justify-between md:block lg:flex">
                        <div class="flex items-baseline text-2xl font-semibold text-indigo-600">
                            {move || {
                                latency()
                                    .map(|x| {
                                        view! { cx, <span>{x.ms} "ms"</span> }
                                    })
                                    .unwrap_or_else(|| {
                                        view! { cx, <span>"..Connecting"</span> }
                                    })
                            }}
                        </div>
                    </dd>
                </div>
            </dl>
        </div>
    }
}

#[component]
fn LadderTable(cx: Scope, ladder: ReadSignal<Option<(Vec<TickData>, Tick)>>) -> impl IntoView {
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
                                <For
                                    each=move || {
                                        if let Some((data, last_traded)) = ladder() {
                                            data.into_iter().map(move |data| {
                                                let is_last_traded = data.tick.0 == last_traded.0;
                                                (data, is_last_traded)}).collect()
                                        } else {
                                            Vec::new()
                                        }
                                    }
                                    key=|(data, is_last_traded)| {
                                        format!("{}-{}-{}-{:?}", data.tick.0, data.back.0, data.lay.0, is_last_traded)
                                    }
                                    view=move |cx, (row, is_last_traded)| {
                                        view! { cx,
                                            <tr class="divide-x divide-gray-200">
                                                <td class="w-1/6 whitespace-nowrap text-sm text-gray-500 sm:pl-0">
                                                    <input type="number" class="w-full"/>
                                                </td>
                                                <td class="w-1/6 whitespace-nowrap text-sm bg-blue-200 text-blue-950">
                                                    {row.back.0.to_string()}
                                                </td>
                                                {if is_last_traded {
                                                    view! { cx,
                                                        <td class="w-1/6 text-center whitespace-nowrap text-sm text-black bg-slate-100">
                                                        {row.tick.0.to_string()}
                                                        </td>
                                                    }
                                                } else {
                                                    view! { cx,
                                                        <td class="w-1/6 text-center whitespace-nowrap text-sm text-white bg-slate-500">
                                                            {row.tick.0.to_string()}
                                                        </td>
                                                    }
                                                }}
                                                <td class="w-1/6 whitespace-nowrap text-sm bg-red-200 text-red-950">
                                                    {row.lay.0.to_string()}
                                                </td>
                                                <td class="w-1/6 whitespace-nowrap text-sm text-gray-500">
                                                    <input type="number" class="w-full"/>
                                                </td>
                                                <td class="w-1/6 text-center whitespace-nowrap text-sm text-gray-500 bg-slate-200">
                                                    {(row.lay.0 + row.back.0).to_string()}
                                                </td>
                                            </tr>
                                        }
                                    }
                                />
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}
