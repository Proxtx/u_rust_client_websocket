use serde::Deserialize;

use crate::http::http;
use crate::{websocket, CompatibilityBehavior};

#[derive(Deserialize)]
struct HttpData {
    export: String,
    arguments: Vec<serde_json::Value>,
}

pub struct Compatibility {
    http: http::Http,
}

#[async_trait::async_trait]
impl CompatibilityBehavior for Compatibility {
    async fn new() -> Self {
        Compatibility {
            http: http::Http::new(),
        }
    }

    async fn execute(
        &mut self,
        websocket: &mut websocket::SocketConnection,
        data: serde_json::Value,
        id: String,
    ) {
        let parsed_data: HttpData;
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
            "request" => {
                let result = self
                    .http
                    .request(
                        parsed_data.arguments[0].as_str().unwrap(),
                        parsed_data.arguments[1].as_str().unwrap(),
                        parsed_data.arguments[2].as_str().unwrap(),
                        parsed_data.arguments[3].as_str().unwrap(),
                    )
                    .await;

                websocket.send(&serde_json::json!({"id": id, "result": result}).to_string())
            }

            _ => {
                println!("Export not found.")
            }
        }
    }
}
