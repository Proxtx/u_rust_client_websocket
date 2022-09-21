mod websocket;
use websocket::SocketConnection;
use serde::{Deserialize};

use std::env;

mod ble;
mod core;

#[derive(Deserialize)]
struct ServiceRequest {
    service: String,
    id: String,
    data: serde_json::Value
}

#[tokio::main]
async fn main() {
    let mut ble_compatibility = ble::compatibility::Compatibility::new().await;
    let mut core_compatibility = core::compatibility::Compatibility::new().await;

    let args: Vec<String> = env::args().collect();

    let mut socket_connection = SocketConnection::new(&args[1]);
    loop {
        let msg = socket_connection.read_message();
        let request_boxed: Result<ServiceRequest, serde_json::Error> = serde_json::from_str(&msg);
        match request_boxed {
            Ok(request) => {
                match request.service.as_str() {
                    "ble" => {
                        ble_compatibility.execute(&mut socket_connection, request.data, request.id).await;
                    }

                    "core" => {
                        core_compatibility.execute(&mut socket_connection, request.data, request.id).await;
                    }

                    _ => {

                    }
                }
            }

            Err(_) => println!("Could not parse the servers message: {}", msg)
        }
    }
}
