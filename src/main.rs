use crate::compatibility::CompatibilityBehavior;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::connect_async;
mod compatibility;
use serde::Deserialize;
mod args;
mod core;
use clap::Parser;

#[cfg(feature = "ble")]
mod ble;
#[cfg(feature = "command")]
mod command;
#[cfg(feature = "http")]
mod http;
#[cfg(feature = "screen")]
mod screen;
#[cfg(feature = "simulate")]
mod simulate;
#[cfg(feature = "win_notification")]
mod win_notification;

mod websocket;

#[derive(Deserialize)]
struct ServiceRequest {
    service: String,
    id: String,
    data: serde_json::Value,
    auth: String,
}

#[tokio::main]
async fn main() {
    let (ws_stream, _) = connect_async("ws://localhost:8080").await.unwrap();
    let (mut socket_sink, mut socket_stream) = ws_stream.split();
    let (socket_sender, socket_receiver) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let mut stream = ReceiverStream::new(socket_receiver).map(Ok);
        socket_sink.send_all(&mut stream).await.unwrap();
    });

    #[cfg(feature = "ble")]
    let ble_module = std::sync::Arc::new(tokio::sync::Mutex::new(
        ble::compatibility::Compatibility::new().await,
    ));

    while let Some(msg_wrapped) = socket_stream.next().await {
        let socket_sender = socket_sender.clone();
        #[cfg(feature = "ble")]
        let ble_module = ble_module.clone();
        tokio::spawn(async move {
            let msg = msg_wrapped.unwrap().to_string();

            let request_boxed: Result<ServiceRequest, serde_json::Error> =
                serde_json::from_str(&msg);
            let request;
            match request_boxed {
                Ok(request_inner) => request = request_inner,
                Err(_) => {
                    println!("Could not parse the servers message: {}", msg);
                    return;
                }
            }

            let mut services = Vec::<String>::new();
            #[cfg(feature = "ble")]
            services.push(String::from("ble"));
            #[cfg(feature = "http")]
            services.push(String::from("http"));
            #[cfg(feature = "win_notification")]
            services.push(String::from("win_notification"));
            #[cfg(feature = "command")]
            services.push(String::from("command"));
            #[cfg(feature = "simulate")]
            services.push(String::from("simulate"));
            #[cfg(feature = "screen")]
            services.push(String::from("screen"));

            let args = args::Args::parse();

            match request.service.as_str() {
                #[cfg(feature = "ble")]
                "ble" => {
                    ble_module
                        .lock()
                        .await
                        .execute(
                            &mut websocket::SocketConnection::new(socket_sender),
                            request.data,
                            request.id,
                        )
                        .await;
                }
                #[cfg(feature = "http")]
                "http" => {
                    http::compatibility::Compatibility::new()
                        .await
                        .execute(
                            &mut websocket::SocketConnection::new(socket_sender),
                            request.data,
                            request.id,
                        )
                        .await
                }
                #[cfg(feature = "command")]
                "command" => {
                    command::compatibility::Compatibility::new()
                        .await
                        .execute(
                            &mut websocket::SocketConnection::new(socket_sender),
                            request.data,
                            request.id,
                        )
                        .await
                }
                #[cfg(feature = "win_notification")]
                "win_notification" => {
                    win_notification::compatibility::Compatibility::new()
                        .await
                        .execute(
                            &mut websocket::SocketConnection::new(socket_sender),
                            request.data,
                            request.id,
                        )
                        .await
                }
                #[cfg(feature = "simulate")]
                "simulate" => {
                    simulate::compatibility::Compatibility::new()
                        .await
                        .execute(
                            &mut websocket::SocketConnection::new(socket_sender),
                            request.data,
                            request.id,
                        )
                        .await
                }
                #[cfg(feature = "screen")]
                "screen" => {
                    screen::compatibility::Compatibility::new()
                        .await
                        .execute(
                            &mut websocket::SocketConnection::new(socket_sender),
                            request.data,
                            request.id,
                        )
                        .await
                }
                "core" => {
                    core::compatibility::Compatibility::new()
                        .await
                        .execute(
                            &mut websocket::SocketConnection::new(socket_sender),
                            request.data,
                            request.id,
                            request.auth == args.auth,
                            &args,
                            services,
                        )
                        .await
                }
                _ => {
                    println!("Could not parse the servers message. {}", &msg);
                }
            }

            println!("Request processed.");
        });
    }
}
