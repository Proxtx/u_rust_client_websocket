use btleplug::api::{Peripheral as _,};

use crate::ble::ble;
use ble::BLEManager;

use crate::websocket;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct BLEData{
  export: String,
  arguments: Vec<serde_json::Value>
}

#[derive(Serialize)]
struct PeripheralDescriptor {
  address: String,
  name: Option<String>,
  address_type: Option<String>,
  tx_power_level: Option<i16>,
}

pub struct Compatibility {
  ble: BLEManager
}

impl Compatibility {
  pub async fn new () -> Self {
    Compatibility {
      ble: BLEManager::new().await
    }
  }

  pub async fn execute (&mut self, websocket: &mut websocket::SocketConnection, data: serde_json::Value, id: String) {
    let parsed_data: BLEData;
    match serde_json::from_value(data) {
      Ok(parsed_data_wrapped) => {
        parsed_data=parsed_data_wrapped;
      }
      Err(_) => {
        return;
      }
    }
    
    println!("{}", parsed_data.export);
    match parsed_data.export.as_str() {
      "start_scan" => {
        websocket.send(&serde_json::json!({"id": id, "result": self.ble.start_scan().await}).to_string());
      }
      "peripherals" => {
        let peripherals = self.ble.peripherals().await;
        let mut peripherals_parsed: Vec<PeripheralDescriptor> = Vec::new();
        for p in peripherals {
          let data = p.properties().await.unwrap().unwrap();

          let address_type = match data.address_type {
            Some(address_type) => if address_type == btleplug::api::AddressType::Public {Option::Some(String::from("public"))} else {Option::Some(String::from("random"))}
            None => Option::None
          };

          peripherals_parsed.push(PeripheralDescriptor {
            address: data.address.to_string(),
            address_type,
            name: data.local_name,
            tx_power_level: data.tx_power_level            
          })
        }
        websocket.send(&serde_json::json!({"id": id, "result": peripherals_parsed}).to_string());
      }
      "connect" => {
        let mut args_iter = parsed_data.arguments.into_iter();
        let arg1: String = serde_json::from_value(args_iter.next().unwrap()).unwrap();
        websocket.send(&serde_json::json!({"id": id, "result": self.ble.connect(&arg1).await}).to_string());
      }
      "discover_services" => {
        websocket.send(&serde_json::json!({"id": id, "result": self.ble.discover_services().await}).to_string())
      }
      "write_to_uuid" => {
        let mut args_iter = parsed_data.arguments.into_iter();
        let arg1: u16 = serde_json::from_value(args_iter.next().unwrap()).unwrap();
        let arg2: Vec<u8> = serde_json::from_value(args_iter.next().unwrap()).unwrap();
        websocket.send(&serde_json::json!({"id": id, "result": self.ble.write_to_uuid(arg1, arg2).await}).to_string());
      }
      "connected_status" => {
        let mut connected = true;
        if let None = self.ble.connected_peripheral {
          connected = false;
        }
        websocket.send(&serde_json::json!({"id": id, "result": connected}).to_string());
      }

      "disconnect" => {
        websocket.send(&serde_json::json!({"id": id, "result": self.ble.disconnect().await}).to_string())
      }

      _ => println!("export not found")
    }
  }
}