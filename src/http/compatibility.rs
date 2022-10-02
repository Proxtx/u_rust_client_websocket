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
        let data: HttpData = serde_json::from_value(data).unwrap();

        match data.export.as_str() {
            "request" => {
                let result = self
                    .http
                    .request(
                        data.arguments[0].as_str().unwrap(),
                        data.arguments[1].as_str().unwrap(),
                        data.arguments[2].as_str().unwrap(),
                    )
                    .await;

                websocket.send(
                    serde_json::json!({"id": id, "result": result})
                        .as_str()
                        .unwrap(),
                )
            }

            _ => {
                println!("Export not found.")
            }
        }
    }
}
