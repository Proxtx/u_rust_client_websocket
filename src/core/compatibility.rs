use serde::{Deserialize};

use crate::websocket;

#[derive(Deserialize)]
struct CoreData{
  export: String,
  arguments: Vec<serde_json::Value>
}

pub struct Compatibility {
  pub restart: bool
}

impl Compatibility {
  pub async fn new () -> Self {
    Compatibility {
      restart: false
    }
  }

  pub async fn execute (&mut self, websocket: &mut websocket::SocketConnection, data: serde_json::Value, id: String, authenticated: bool) {
    let args: Vec<String> = std::env::args().collect();

    let parsed_data: CoreData = serde_json::from_value(data).unwrap();
    println!("{}", parsed_data.export);

    if !authenticated && parsed_data.export != "id" {
      return;
    }

    match parsed_data.export.as_str() {
      "services" => {
        parsed_data.arguments.iter();
        websocket.send(&serde_json::json!({"id": id, "result": ["core", "ble"]}).to_string());
      },
      "id" => {
        websocket.send(&serde_json::json!({"id": id, "result": args[1]}).to_string());
      },
      "key" => {
        websocket.send(&serde_json::json!({"id": id, "result": args[2]}).to_string());
      },
      "restart" => {
        self.restart = true;
        websocket.send(&serde_json::json!({"id": id}).to_string());
      }
      _ => println!("Export not found!") 
    }
  }
}