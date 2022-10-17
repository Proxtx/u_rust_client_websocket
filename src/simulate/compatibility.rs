use crate::compatibility::CompatibilityBehavior;
use crate::simulate::simulate::Simulate;
use crate::websocket;
use serde::Deserialize;

#[derive(Deserialize)]
struct SimulateData {
    export: String,
    arguments: Vec<serde_json::Value>,
}

pub struct Compatibility {
    simulate: Simulate,
}

#[async_trait::async_trait]
impl CompatibilityBehavior for Compatibility {
    async fn new() -> Self {
        Compatibility {
            simulate: Simulate::new(),
        }
    }

    async fn execute(
        &mut self,
        socket: &mut websocket::SocketConnection,
        data: serde_json::Value,
        id: String,
    ) {
        let parsed_data: SimulateData;
        match serde_json::from_value(data) {
            Ok(parsed_data_wrapped) => {
                parsed_data = parsed_data_wrapped;
            }
            Err(_) => {
                return;
            }
        }

        println!("{}", parsed_data.export);

        match parsed_data.export.as_str() {
            "keys" => {
                let result = self
                    .simulate
                    .keys(&parsed_data.arguments[0].to_owned().to_string())
                    .await;
                socket.send(&serde_json::json!({"id": id, "result": result}).to_string())
            }
            _ => {
                println!("Export not found.")
            }
        }
    }
}
