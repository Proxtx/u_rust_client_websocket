use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType, Characteristic};
use btleplug::platform::{Adapter, Manager, Peripheral};

pub struct BLEManager {
  scanning: bool,
  central: Adapter,
  pub connected_peripheral: Option<Peripheral>
}

impl BLEManager {
  pub async fn new() -> Self {
    
    let manager = Manager::new().await.unwrap();
    let adapters = manager.adapters().await.unwrap();
    let central = adapters.into_iter().nth(0).unwrap();


    BLEManager {
      scanning: false,
      central,
      connected_peripheral: Option::None
    }
  }

  pub async fn start_scan (&mut self) {
    self.central.start_scan(ScanFilter::default()).await.unwrap();
    self.scanning = true;
  }

  pub async fn peripherals(&mut self) -> Vec<Peripheral> {
    self.central.peripherals().await.unwrap()
  }

  pub async fn connect(&mut self, address: &str) -> bool {
    match self.connected_peripheral {
      Some(_) => {
        return false;
      }
      None => {
      }
    }

    for p in self.peripherals().await {
      if p.properties().await.unwrap().unwrap().address.to_string() == address {
        match p.connect().await {
          Ok(_) => {
            self.connected_peripheral = Option::Some(p);
            return true
          }

          Err(_) => {
            return false
          }
        }
      }
    }

    false
  }

  pub async fn discover_services (&mut self) -> bool {
    match &self.connected_peripheral {
      Some(peripheral) => {
        peripheral.discover_services().await.unwrap();
        return true;
      }

      None => {
        return false;
      }
    }
  }

  pub async fn write_to_uuid (&mut self, uuid: u16, content: Vec<u8>) -> bool {
    if let None = self.connected_peripheral {
      return false
    }

    let peripheral = self.connected_peripheral.as_ref().unwrap();
    let cmd_char: &Characteristic;
    let characteristics = peripheral.characteristics();

    match characteristics.iter().find(|c| c.uuid == uuid_from_u16(uuid)) {
      None => {
        return false;
      }

      Some(inner_cmd_char) => {
        cmd_char = inner_cmd_char;
      }
    }

    match peripheral.write(&cmd_char, &content, WriteType::WithoutResponse).await {
      Ok(_) => {
        return true;
      }
      
      Err(_) => {
        return false;
      }
    }
  }

  pub async fn disconnect (&mut self) -> bool {
    match &self.connected_peripheral {
      Some(peripheral) => {
        peripheral.disconnect().await.unwrap();
        self.connected_peripheral = Option::None;
        return true;
      }

      None => {
        return false;
      }
    }
  }
}