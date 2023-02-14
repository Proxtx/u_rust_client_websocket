use btleplug::api::{
    Central, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};

pub struct BLEManager {
    scanning: bool,
    central: Adapter,
    pub connected_peripheral: Option<Peripheral>,
}

pub struct Result {
    pub success: bool,
    pub error: Option<String>,
}

fn error_to_string(error: btleplug::Error) -> String {
    match error {
        btleplug::Error::PermissionDenied => String::from("Permission Denied"),
        btleplug::Error::DeviceNotFound => String::from("Device not found"),
        btleplug::Error::NotConnected => String::from("Not Connected"),
        btleplug::Error::InvalidBDAddr(_) => String::from("Invalid BD Address"),
        btleplug::Error::NotSupported(error) => String::from(format!("Not Supported: {}", error)),
        btleplug::Error::TimedOut(time) => {
            String::from(format!("Time out waited {} seconds", time.as_secs()))
        }
        btleplug::Error::Uuid(_) => String::from("Uuid error"),
        btleplug::Error::Other(_) => String::from("Other Error"),
    }
}

impl BLEManager {
    pub async fn new() -> Self {
        let manager = Manager::new().await.unwrap();
        let adapters = manager.adapters().await.unwrap();
        let central = adapters.into_iter().nth(0).unwrap();

        BLEManager {
            scanning: false,
            central,
            connected_peripheral: Option::None,
        }
    }

    pub async fn start_scan(&mut self) -> Result {
        match self.central.start_scan(ScanFilter::default()).await {
            Ok(_) => {}
            Err(error) => {
                if self.scanning {
                    return Result {
                        success: true,
                        error: Option::None,
                    };
                }
                return Result {
                    success: false,
                    error: Option::Some(error_to_string(error)),
                };
            }
        };
        self.scanning = true;

        return Result {
            success: true,
            error: Option::None,
        };
    }

    pub async fn peripherals(&mut self) -> Vec<Peripheral> {
        self.central.peripherals().await.unwrap()
    }

    pub async fn connect(&mut self, address: &str) -> Result {
        match self.connected_peripheral {
            Some(_) => {
                return Result {
                    success: false,
                    error: Option::Some(String::from("Already connected")),
                };
            }
            None => {}
        }

        for p in self.peripherals().await {
            if p.properties().await.unwrap().unwrap().address.to_string() == address {
                match p.connect().await {
                    Ok(_) => {
                        self.connected_peripheral = Option::Some(p);
                        return Result {
                            success: true,
                            error: Option::None,
                        };
                    }

                    Err(error) => {
                        return Result {
                            success: false,
                            error: Option::Some(error_to_string(error)),
                        }
                    }
                }
            }
        }

        Result {
            success: false,
            error: Option::Some(String::from("Unknown Error")),
        }
    }

    pub async fn discover_services(&mut self) -> Result {
        match &self.connected_peripheral {
            Some(peripheral) => {
                peripheral.discover_services().await.unwrap();
                return Result {
                    success: true,
                    error: Option::None,
                };
            }

            None => {
                return Result {
                    success: false,
                    error: Option::Some(String::from("Not connected")),
                };
            }
        }
    }

    pub async fn write_to_uuid(&mut self, uuid: uuid::Uuid, content: Vec<u8>) -> Result {
        if let None = self.connected_peripheral {
            return Result {
                success: false,
                error: Option::Some(String::from("Not connected")),
            };
        }

        let peripheral = self.connected_peripheral.as_ref().unwrap();
        let cmd_char: &Characteristic;
        let characteristics = peripheral.characteristics();

        match characteristics.iter().find(|c| c.uuid == uuid) {
            None => {
                return Result {
                    success: false,
                    error: Option::Some(String::from("Characteristic not found")),
                };
            }

            Some(inner_cmd_char) => {
                cmd_char = inner_cmd_char;
            }
        }

        match peripheral
            .write(&cmd_char, &content, WriteType::WithoutResponse)
            .await
        {
            Ok(_) => {
                return Result {
                    success: true,
                    error: Option::None,
                };
            }

            Err(error) => {
                return Result {
                    success: false,
                    error: Option::Some(error_to_string(error)),
                }
            }
        }
    }

    pub async fn disconnect(&mut self) -> Result {
        match &self.connected_peripheral {
            Some(peripheral) => match peripheral.disconnect().await {
                Ok(_) => {
                    self.connected_peripheral = Option::None;
                    return Result {
                        success: true,
                        error: Option::None,
                    };
                }
                Err(error) => {
                    self.connected_peripheral = Option::None;
                    return Result {
                        success: false,
                        error: Option::Some(error_to_string(error)),
                    };
                }
            },

            None => {
                return Result {
                    success: false,
                    error: Option::Some(String::from("Not connected")),
                };
            }
        }
    }
}
