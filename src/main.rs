use clap::Parser;
use serde::Deserialize;
use websocket::SocketConnection;

mod args;
mod ble;
mod core;
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
    while run().await {}
}

async fn run() -> bool {
    #[cfg(feature = "ble")]
    let mut ble_compatibility = ble::compatibility::Compatibility::new().await;
    let mut core_compatibility = core::compatibility::Compatibility::new().await;

    let args = args::Args::parse();

    let mut socket_connection = SocketConnection::new(&args.ws_url);

    println!("System initialized!");

    loop {
        let msg = socket_connection.read_message();
        let request_boxed: Result<ServiceRequest, serde_json::Error> = serde_json::from_str(&msg);
        match request_boxed {
            Ok(request) => match request.service.as_str() {
                #[cfg(feature = "ble")]
                "ble" => {
                    if request.auth != args.auth {
                        continue;
                    }
                    ble_compatibility
                        .execute(&mut socket_connection, request.data, request.id)
                        .await;
                }

                "core" => {
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
                }

                _ => {}
            },

            Err(_) => println!("Could not parse the servers message: {}", msg),
        }

        println!("Request processed.")
    }
}
