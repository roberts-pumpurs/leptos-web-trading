use std::borrow::Cow;
use std::ops::ControlFlow;

use axum::extract::ws::{CloseFrame, Message};
use axum::extract::WebSocketUpgrade;
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};

pub async fn handler(
    ws: WebSocketUpgrade,
    // State(state): State<crate::app::WebAppState<T>>,
) -> impl IntoResponse {
    let who = "who";
    ws.on_upgrade(move |agent_socket| async move {
        let (mut sender, mut receiver) = agent_socket.split();

        // Spawn a task that will push several messages to the client (does not matter what client
        // does)
        let mut send_task = tokio::spawn(async move {
            loop {
                // In case of any websocket error, we exit.
                if sender.send(Message::Text(format!("Hi from server"))).await.is_err() {
                    break
                }

                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            }

            if let Err(e) = sender
                .send(Message::Close(Some(CloseFrame {
                    code: axum::extract::ws::close_code::NORMAL,
                    reason: Cow::from("Goodbye"),
                })))
                .await
            {
                println!("Could not send Close due to {}, probably it is ok?", e);
            }
        });

        // This second task will receive messages from client and print them on server console
        let mut recv_task = tokio::spawn(async move {
            let mut cnt = 0;
            while let Some(Ok(msg)) = receiver.next().await {
                cnt += 1;
                // print message and break if instructed to do so
                if process_message(msg, who.to_string()).is_break() {
                    break
                }
            }
            cnt
        });

        // If any one of the tasks exit, abort the other.
        tokio::select! {
            rv_a = (&mut send_task) => {
                match rv_a {
                    Ok(_) => println!("Messages sent to {}", who),
                    Err(a) => println!("Error sending messages {:?}", a)
                }
                recv_task.abort();
            },
            rv_b = (&mut recv_task) => {
                match rv_b {
                    Ok(b) => println!("Received {} messages", b),
                    Err(b) => println!("Error receiving messages {:?}", b)
                }
                send_task.abort();
            }
        }

        // returning from the handler closes the websocket connection
        println!("Websocket context {} destroyed", who);
    })
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: String) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {} sent str: {:?}", who, t);
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(">>> {} sent close with code {} and reason `{}`", who, cf.code, cf.reason);
            } else {
                println!(">>> {} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(())
        }

        Message::Pong(v) => {
            println!(">>> {} sent pong with {:?}", who, v);
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {} sent ping with {:?}", who, v);
        }
    }
    ControlFlow::Continue(())
}
