mod websocket;
use websocket::SocketConnection;
use serde::{Deserialize};

mod ble;
mod core;

#[derive(Deserialize)]
struct ServiceRequest {
    service: String,
    id: String,
    data: serde_json::Value,
    auth: String
}
#[tokio::main]
async fn main () {
    while run().await {}
}

async fn run() -> bool {
    let mut ble_compatibility = ble::compatibility::Compatibility::new().await;
    let mut core_compatibility = core::compatibility::Compatibility::new().await;

    let args: Vec<String> = std::env::args().collect();

    let mut socket_connection = SocketConnection::new(&args[4]);

    println!("System initialized!");

    loop {
        let msg = socket_connection.read_message();
        let request_boxed: Result<ServiceRequest, serde_json::Error> = serde_json::from_str(&msg);
        match request_boxed {
            Ok(request) => {
                match request.service.as_str() {
                    "ble" => {
                        if request.auth != args[3] {
                            continue;
                        }
                        ble_compatibility.execute(&mut socket_connection, request.data, request.id).await;
                    }

                    "core" => {
                        core_compatibility.execute(&mut socket_connection, request.data, request.id, request.auth == args[3]).await;
                        if core_compatibility.restart {
                            return true;
                        }
                    }

                    _ => {

                    }
                }
            }

            Err(_) => println!("Could not parse the servers message: {}", msg)
        }

        println!("Request processed.")
    }
}
