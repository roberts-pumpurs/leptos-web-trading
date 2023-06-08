use std::sync::{Arc, RwLock};

use futures::channel::oneshot;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use leptos::*;
use leptos_router::*;

#[component]
pub fn LadderView(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_else(|| "1".into()));

    let (_chat_client, set_chat_client) =
        create_signal::<Option<Arc<RwLock<SplitSink<WebSocket, Message>>>>>(cx, None);
    let (_chat_messages, set_chat_messages) = create_signal::<Vec<String>>(cx, Vec::new());

    create_effect(cx, move |_| {
        let host = window().location().host().unwrap_or("127.0.0.1:3000".to_string());
        let protocol = {
            if window().location().protocol().unwrap_or("http".to_string()).contains("https") {
                "wss"
            } else {
                "ws"
            }
        };
        let url = format!("{}://{}/ws/{}", protocol, host, id());
        if let Ok(connection) = WebSocket::open(&url) {
            let (sender, mut recv) = connection.split();
            let (close_sender, mut close_recv) = oneshot::channel::<()>();
            spawn_local(async move {
                while let Some(msg) = recv.next().await {
                    match msg {
                        Ok(Message::Text(msg)) => {
                            set_chat_messages.update(|msgs| msgs.push(msg));
                        }
                        _ => break,
                    }
                    // NOTE: we cannot use futures::select! here because of some weird
                    // FutureFused trait bounds not being available for `recv`.
                    // The implication is that the Close frame never gets sent to the server.
                    if close_recv.try_recv().is_err() {
                        log!("chat is disconnecting from server");
                        break
                    }
                }
            });

            let sender = Arc::new(RwLock::new(sender));

            let sender_clone = sender.clone();
            on_cleanup(cx, move || {
                spawn_local(async move {
                    let mut client = sender_clone.write().unwrap();
                    client.close().await.unwrap();
                    let _ = close_sender.send(());
                });
            });
            set_chat_client(Some(sender))
        } else {
            log!("chat failed to connect to server");
        }
    });

    let _send_msg_to_server = move |msg: String| {
        set_chat_client.update(|client| {
            if let Some(client) = client {
                let client = client.clone();
                spawn_local(async move {
                    let mut client = client.write().unwrap();
                    let _ = client.send(Message::Text(msg)).await;
                });
            }
        });
    };

    view! { cx,
        <div class="HomeView">
            <LadderTable/>
        </div>
    }
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
