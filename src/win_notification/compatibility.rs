use crate::compatibility::CompatibilityBehavior;
use crate::websocket;
use crate::win_notification::win_notification::{WinNotification, WinNotificationConfiguration};
use serde::Deserialize;

#[derive(Deserialize)]
struct WinNotificationData {
    export: String,
    arguments: Vec<serde_json::Value>,
}

pub struct Compatibility {
    win_notification: WinNotification,
}

#[async_trait::async_trait]
impl CompatibilityBehavior for Compatibility {
    async fn new() -> Self {
        Compatibility {
            win_notification: WinNotification::new(),
        }
    }

    async fn execute(
        &mut self,
        socket: &mut websocket::SocketConnection,
        data: serde_json::Value,
        id: String,
    ) {
        let parsed_data: WinNotificationData;
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
            "notification" => {
                let config: WinNotificationConfiguration =
                    serde_json::from_value(parsed_data.arguments[0].to_owned()).unwrap();
                let result = self.win_notification.notification(config);
                socket
                    .send(&serde_json::json!({"id": id, "result": result}).to_string())
                    .await
            }
            _ => {
                println!("Export not found.")
            }
        }
    }
}
