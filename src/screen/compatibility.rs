use crate::compatibility::CompatibilityBehavior;
use crate::screen::screen::Screen;
use crate::websocket;
use serde::Deserialize;

#[derive(Deserialize)]
struct ScreenData {
    export: String,
    arguments: Vec<serde_json::Value>,
}

pub struct Compatibility {
    screen: Screen,
}

#[async_trait::async_trait]
impl CompatibilityBehavior for Compatibility {
    async fn new() -> Self {
        Compatibility {
            screen: Screen::new(),
        }
    }

    async fn execute(
        &mut self,
        socket: &mut websocket::SocketConnection,
        data: serde_json::Value,
        id: String,
    ) {
        let parsed_data: ScreenData;
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
            "screenshot" => {
                let mut args_iter = parsed_data.arguments.into_iter();
                let arg1: u8 = serde_json::from_value(args_iter.next().unwrap()).unwrap();

                let result = self.screen.screenshot(arg1.into());
                socket.send(&serde_json::json!({"id": id, "result": result}).to_string())
            }
            "screens" => socket
                .send(&serde_json::json!({"id": id, "result": self.screen.screens()}).to_string()),
            _ => {
                println!("Export not found.")
            }
        }
    }
}
