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
#[cfg(feature = "http")]
mod http;

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
    #[cfg(features = "http")]
    service_map.insert(
        String::from("http"),
        Box::from(http::compatibility::Compatibility::new().await),
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
                )
                .await;
            if core_compatibility.restart {
                return true;
            }
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
