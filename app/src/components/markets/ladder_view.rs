use std::sync::{Arc, RwLock};

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use leptos::*;

#[component]
pub fn LadderView(cx: Scope) -> impl IntoView {
    let (chat_client, set_chat_client) =
        create_signal::<Option<Arc<RwLock<SplitSink<WebSocket, Message>>>>>(cx, None);
    let (chat_messages, set_chat_messages) = create_signal::<Vec<String>>(cx, Vec::new());

    create_effect(cx, move |_| {
        tracing::info!("chat is connecting to server");
        let connection =
            WebSocket::open("ws://127.0.0.1:3000/ws").expect("Failed to connect to WS stream");
        let (sender, mut recv) = connection.split();
        spawn_local(async move {
            while let Some(msg) = recv.next().await {
                match msg {
                    Ok(Message::Text(msg)) => {
                        set_chat_messages.update(|msgs| msgs.push(msg));
                    }
                    _ => {
                        break
                    }
                }
            }
        });

        set_chat_client(Some(Arc::new(RwLock::new(sender))))
    });

    let send_msg_to_server = move |msg: String| {
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
            <button on:click=move |_| send_msg_to_server("Hello from client".to_owned())>
                "Send message to server"
            </button>
            <For
                each=move || { chat_messages.get().into_iter().enumerate() }
                key=|(index, _error)| *index
                view=move |cx, msg| {
                    view! { cx, <p>{msg}</p> }
                }
            />
        </div>
    }
}
