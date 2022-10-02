use crate::websocket;
use serde::Deserialize;

use crate::args::Args;

#[derive(Deserialize)]
struct CoreData {
    export: String,
    #[allow(dead_code)]
    arguments: Vec<serde_json::Value>,
}

pub struct Compatibility {
    pub restart: bool,
}

impl Compatibility {
    pub async fn new() -> Self {
        Compatibility { restart: false }
    }

    pub async fn execute(
        &mut self,
        websocket: &mut websocket::SocketConnection,
        data: serde_json::Value,
        id: String,
        authenticated: bool,
        args: &Args,
        services: Vec<String>,
    ) {
        let parsed_data: CoreData;
        match serde_json::from_value(data) {
            Ok(parsed_data_wrapped) => {
                parsed_data = parsed_data_wrapped;
            }
            Err(_) => {
                return;
            }
        }

        println!("{}", parsed_data.export);
        if !authenticated && (parsed_data.export != "id" && parsed_data.export != "key") {
            return;
        }

        match parsed_data.export.as_str() {
            "services" => {
                websocket.send(&serde_json::json!({"id": id, "result": services}).to_string());
            }
            "id" => {
                websocket.send(&serde_json::json!({"id": id, "result": args.id}).to_string());
            }
            "key" => {
                websocket.send(&serde_json::json!({"id": id, "result": args.key}).to_string());
            }
            "restart" => {
                self.restart = true;
                websocket.send(&serde_json::json!({ "id": id }).to_string());
            }
            _ => println!("Export not found!"),
        }
    }
}
