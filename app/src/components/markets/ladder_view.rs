use std::collections::HashMap;

use futures::{FutureExt, SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use gloo_timers;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;
use leptos_router::*;
use rust_decimal_macros::dec;
use trading_types::common::{Order, RequestId, Side, Size};
use trading_types::from_server::{Latency, ServerMessage, TickData, TraderOrders};
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

#[derive(Debug, Clone, PartialEq)]
struct TickDataWrapper {
    id: usize,
    tick_data: RwSignal<TickData>,
    is_last_traded: RwSignal<bool>,
}

#[component]
fn LadderViewInternal(cx: Scope, id: Memo<u32>) -> impl IntoView {
    let derived_ws_url = create_memo::<String>(cx, move |_| derive_ws_url(id()));
    let (latency, set_latency) = create_signal::<Option<Latency>>(cx, None);
    let (ladder, set_ladder) = create_signal::<Vec<TickDataWrapper>>(cx, vec![]);
    let (trader_orders, set_trader_orders) = create_signal::<TraderOrders>(
        cx,
        TraderOrders { matched_orders: HashMap::new(), unmatched_orders: HashMap::new() },
    );
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
                                                let res = set.into_iter().enumerate().map(|(idx, data)| {
                                                    let tick_data = create_rw_signal(cx, data);
                                                    let is_last_traded = create_rw_signal(cx, false);

                                                    TickDataWrapper {
                                                        id: idx,
                                                        tick_data,
                                                        is_last_traded,
                                                    }
                                                }).collect::<Vec<_>>();
                                                set_ladder(res);
                                            },
                                            ServerMessage::TickUpdate(new_value) => {
                                                set_ladder.update(|ladder| {
                                                    ladder.iter_mut().for_each(|x| {
                                                        if x.tick_data.get_untracked().tick == new_value.tick {
                                                            x.tick_data.update(|prev| {
                                                                *prev = new_value.clone();
                                                            });
                                                        }
                                                    });
                                                });
                                            },
                                            ServerMessage::NewLatestMatch(new_value) => {
                                                set_ladder.update(|ladder| {
                                                    ladder.iter_mut().for_each(|x| {
                                                        if x.tick_data.get_untracked().tick == new_value.tick {
                                                            x.is_last_traded.update(|prev| {
                                                                *prev = true;
                                                            });
                                                        } else {
                                                            x.is_last_traded.update(|prev| {
                                                                *prev = false;
                                                            });
                                                        }
                                                    });
                                                });
                                            },
                                            ServerMessage::OrderStateUpdate(new_order_state) => {
                                                set_trader_orders.update(|order_state| {
                                                    *order_state = new_order_state;
                                                });
                                            },
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
                    set_ladder(vec![]);
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
            <StatsComponent latency=latency trader_orders=trader_orders/>
            <OrderInformation trader_orders=trader_orders/>
            <LadderTable ladder=ladder ws_client_sender=ws_client_sender/>
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
fn StatsComponent(
    cx: Scope,
    latency: ReadSignal<Option<Latency>>,
    trader_orders: ReadSignal<TraderOrders>,
) -> impl IntoView {
    view! { cx,
        <div>
            <h3 class="text-base font-semibold leading-6 text-gray-900">"Stats"</h3>
            <dl class="mt-5 grid grid-cols-3 divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow md:grid-cols-3 md:divide-x md:divide-y-0">
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
                <div class="px-4 py-5 sm:p-6">
                    <dt class="text-base font-normal text-gray-900">"Matched backs"</dt>
                    <dd class="mt-1 flex items-baseline justify-between md:block lg:flex">
                        <div class="flex items-baseline text-2xl font-semibold text-indigo-600">
                            {move || {
                                let orders = trader_orders();
                                let matched = orders
                                    .matched_orders
                                    .iter()
                                    .filter(|(_, x)| x.side == Side::Back)
                                    .fold(dec!(0), |acc, (_, x)| acc + x.size.0);
                                view! { cx, <span>{matched.to_string()} " €"</span> }
                            }}
                        </div>
                    </dd>
                </div>
                <div class="px-4 py-5 sm:p-6">
                    <dt class="text-base font-normal text-gray-900">"Matched lays"</dt>
                    <dd class="mt-1 flex items-baseline justify-between md:block lg:flex">
                        <div class="flex items-baseline text-2xl font-semibold text-indigo-600">
                            {move || {
                                let orders = trader_orders();
                                let matched = orders
                                    .matched_orders
                                    .iter()
                                    .filter(|(_, x)| x.side == Side::Lay)
                                    .fold(dec!(0), |acc, (_, x)| acc + x.size.0);
                                view! { cx, <span>{matched.to_string()} " €"</span> }
                            }}
                        </div>
                    </dd>
                </div>
            </dl>
        </div>
    }
}

#[component]
fn OrderInformation(cx: Scope, trader_orders: ReadSignal<TraderOrders>) -> impl IntoView {
    view! { cx,
        <div class="px-4 sm:px-6 lg:px-8">
            <div class="sm:flex sm:items-center">
                <div class="sm:flex-auto">
                    <h1 class="text-base font-semibold leading-6 text-gray-900">"Unmatched bets"</h1>
                    <p class="mt-2 text-sm text-gray-700">"A list of all unmatched orders"</p>
                </div>
            </div>
            <div class="-mx-4 mt-8 sm:-mx-0">
                <table class="min-w-full divide-y divide-gray-300">
                    <thead>
                        <tr>
                            <th
                                scope="col"
                                class=" px-3 py-3.5 text-left text-sm font-semibold text-gray-900 sm:table-cell"
                            >
                                "Tick"
                            </th>
                            <th
                                scope="col"
                                class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-0"
                            >
                                "Side"
                            </th>
                            <th
                                scope="col"
                                class=" px-3 py-3.5 text-left text-sm font-semibold text-gray-900 lg:table-cell"
                            >
                                "Size"
                            </th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-200 bg-white">
                        {move || {
                            let mut res = trader_orders()
                                .unmatched_orders
                                .values()
                                .map(|(order)| {
                                    (
                                        view! { cx,
                                            <tr>
                                            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500 sm:table-cell">
                                                {order.tick.0.to_string()}
                                            </td>
                                                <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-0">
                                                    {order.side.to_string()}
                                                </td>
                                                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500 sm:table-cell">
                                                    {order.size.0.to_string()}
                                                </td>
                                            </tr>
                                        },
                                        (order.clone()),
                                    )
                                })
                                .collect::<Vec<_>>();
                            res.sort_by(|(_, order1), (_, order2)| {
                                order1
                                    .tick
                                    .cmp(&order2.tick)
                                    .then_with(|| {
                                        order1
                                            .side
                                            .cmp(&order2.side)
                                            .then_with(|| {
                                                order1.size.cmp(&order2.size)
                                            })
                                    })
                            });
                            res.into_iter().map(|(x, _)| x).collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn LadderTable(
    cx: Scope,
    #[prop(into)] ladder: Signal<Vec<TickDataWrapper>>,
    ws_client_sender: Memo<Option<SenderWrapper>>,
) -> impl IntoView {
    view! { cx,
        <div class="px-4 sm:px-6 lg:px-8">
            <div class="mt-8 flow-root">
                <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div class="h-96 overflow-y-auto">
                        <table class="table-fixed w-full divide-y divide-gray-300">
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
                                {move || {
                                    view! { cx,
                                        <For
                                            each=ladder
                                            key=|val| { val.id }
                                            view=move |cx, data| {
                                                view! { cx, <TickRow data=data ws_client_sender=ws_client_sender/> }
                                            }
                                        />
                                    }
                                        .into_view(cx)
                                }}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TickRow(
    cx: Scope,
    data: TickDataWrapper,
    ws_client_sender: Memo<Option<SenderWrapper>>,
) -> impl IntoView {
    let input_element_back: NodeRef<Input> = create_node_ref(cx);
    let input_element_lay: NodeRef<Input> = create_node_ref(cx);

    let on_submit_core = move |node_ref: NodeRef<Input>, side: Side| {
        // here, we'll extract the value from the input
        let Some(value) = node_ref() else {
            return;
        };
        let value = value.value();
        let Ok(value) = value.parse::<rust_decimal::Decimal>() else {
            return;
        };
        let tick = data.tick_data.get().tick;
        let order = Order { tick, size: Size(value), side };

        let sender = ws_client_sender();
        if let Some(mut sender) = sender {
            use uuid::Uuid;
            let request_id = RequestId(Uuid::new_v4().to_string());
            spawn_local(async move {
                let _ =
                    sender.sender.send(Some(TraderMessage::PlaceOrder(request_id, order))).await;
            });
        };
    };
    let on_submit_back = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        on_submit_core(input_element_back, Side::Back);
    };

    let on_submit_lay = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        on_submit_core(input_element_lay, Side::Lay);
    };

    view! { cx,
        <tr class="divide-x divide-gray-200">
            <td class="w-1/6 whitespace-nowrap text-sm text-gray-500 sm:pl-0">
                <form on:submit=on_submit_back>
                    <input type="number" class="w-full" node_ref=input_element_back/>
                </form>
            </td>
            {move || {
                let row = data.tick_data.get();
                let is_last_traded = data.is_last_traded.get();
                view! { cx,
                    <td class="w-1/6 whitespace-nowrap text-sm bg-blue-200 text-blue-950">
                        {row.available_backs.0.to_string()}
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
                        {row.available_lays.0.to_string()}
                    </td>
                }
            }}
            <td class="w-1/6 whitespace-nowrap text-sm text-gray-500">
                <form on:submit=on_submit_lay>
                    <input type="number" class="w-full" node_ref=input_element_lay/>
                </form>
            </td>
            <td class="w-1/6 text-center whitespace-nowrap text-sm text-gray-500 bg-slate-200">
                {move || {
                    let data = data.tick_data.get();
                    let matched_liquidity = data.total_matched.0;
                    let total_liquidity = matched_liquidity + data.available_backs.0
                        + data.available_lays.0;
                    if total_liquidity == dec!(0) {
                        return view! { cx,
                                <progress value="0" max="100">
                                    "0%"
                                </progress>
                            };
                    }
                    let percentage_matched = matched_liquidity / total_liquidity * dec!(100);
                    view! { cx,
                        <progress value=matched_liquidity.to_string() max=total_liquidity.to_string()>
                            {percentage_matched.to_string()}
                            "%"
                        </progress>
                    }
                }}
            </td>
        </tr>
    }
}
