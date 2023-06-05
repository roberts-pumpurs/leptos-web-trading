use leptos::*;

#[component]
pub fn LadderView(cx: Scope) -> impl IntoView {
    let (chat_messages, set_chat_messages) = create_signal::<Vec<String>>(cx, Vec::new());

    #[cfg(not(feature = "ssr"))]
    let client = {
        use std::sync::{Arc, RwLock};

        use futures::stream::SplitSink;
        use futures::{SinkExt, StreamExt};
        use gloo_net::websocket::futures::WebSocket;
        use gloo_net::websocket::Message;

        let connection =
            WebSocket::open("ws://127.0.0.1:3000/ws").expect("Failed to connect to WS stream");
        let (sender, mut recv) = connection.split();
        let sender = Arc::new(RwLock::new(sender));
        let sender_clone = sender.clone();
        spawn_local(async move {
            while let Some(msg) = recv.next().await {
                match msg {
                    Ok(Message::Text(msg)) => {
                        set_chat_messages.update(|msgs| msgs.push(msg));
                    }
                    _ => break,
                }
            }
            let sender = sender_clone.clone();
            spawn_local(async move {
                let mut sender = sender.write().unwrap();
                let _ = sender.close().await;
            });
        });

        let sender_clone = sender.clone();
        on_cleanup(cx, move || {
                spawn_local(async move {
                    let mut client = sender_clone.write().unwrap();
                    let _ = client.close().await;
                });
        });

        sender
    };

    #[cfg(not(feature = "ssr"))]
    let send_msg_to_server = move |msg: String| {
        use futures::{SinkExt, StreamExt};
        use gloo_net::websocket::Message;
        let client = client.clone();
        spawn_local(async move {
            let mut client = client.write().unwrap();
            let _ = client.send(Message::Text(msg)).await;
        });
    };

    #[cfg(feature = "ssr")]
    let send_msg_to_server = move |_| {};

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
