#![windows_subsystem = "windows"]

use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use websocket::SocketConnection;

use crate::compatibility::CompatibilityBehavior;

mod args;
mod compatibility;
mod core;
mod websocket;

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

#[derive(Deserialize)]
struct ServiceRequest {
    service: String,
    id: String,
    data: serde_json::Value,
    auth: String,
}

#[tokio::main]
async fn main() {
    while run().await {}
}

async fn run() -> bool {
    let mut service_map: HashMap<String, Box<dyn CompatibilityBehavior>> = HashMap::new();

    #[cfg(feature = "ble")]
    service_map.insert(
        String::from("ble"),
        Box::from(ble::compatibility::Compatibility::new().await),
    );
    #[cfg(feature = "http")]
    service_map.insert(
        String::from("http"),
        Box::from(http::compatibility::Compatibility::new().await),
    );

    #[cfg(feature = "win_notification")]
    service_map.insert(
        String::from("win_notification"),
        Box::from(win_notification::compatibility::Compatibility::new().await),
    );

    #[cfg(feature = "command")]
    service_map.insert(
        String::from("command"),
        Box::from(command::compatibility::Compatibility::new().await),
    );

    #[cfg(feature = "simulate")]
    service_map.insert(
        String::from("simulate"),
        Box::from(simulate::compatibility::Compatibility::new().await),
    );
    #[cfg(feature = "screen")]
    service_map.insert(
        String::from("screen"),
        Box::from(screen::compatibility::Compatibility::new().await),
    );

    let mut core_compatibility = core::compatibility::Compatibility::new().await;

    let args = args::Args::parse();

    let mut socket_connection = SocketConnection::new(&args.ws_url);

    println!("System initialized!");

    loop {
        let msg = socket_connection.read_message();
        let request_boxed: Result<ServiceRequest, serde_json::Error> = serde_json::from_str(&msg);
        let request;
        match request_boxed {
            Ok(request_inner) => request = request_inner,
            Err(_) => {
                println!("Could not parse the servers message: {}", msg);
                continue;
            }
        }

        if request.service == "core" {
            core_compatibility
                .execute(
                    &mut socket_connection,
                    request.data,
                    request.id,
                    request.auth == args.auth,
                    &args,
                    service_map.keys().cloned().collect(),
                )
                .await;
            if core_compatibility.restart {
                return true;
            }
            println!("Request processed.");
            continue;
        }

        if service_map.contains_key(&request.service) {
            service_map
                .get_mut(&request.service)
                .unwrap()
                .execute(&mut socket_connection, request.data, request.id)
                .await;
        }

        println!("Request processed.")
    }
}
