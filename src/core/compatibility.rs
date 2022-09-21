use serde::{Deserialize};

use crate::websocket;

#[derive(Deserialize)]
struct CoreData{
  export: String,
  arguments: Vec<serde_json::Value>
}

pub struct Compatibility {

}

impl Compatibility {
  pub async fn new () -> Self {
    Compatibility {

    }
  }

  pub async fn execute (&mut self, websocket: &mut websocket::SocketConnection, data: serde_json::Value, id: String) {
    let parsed_data: CoreData = serde_json::from_value(data).unwrap();
    println!("{}", parsed_data.export);
    match parsed_data.export.as_str() {
      "services" => {
        parsed_data.arguments.iter();
        websocket.send(&serde_json::json!({"id": id, "result": ["core", "ble"]}).to_string());
      }
      _ => println!("Export not found!") 
    }
  }
}