use crate::command::command::{Command, CommandConfiguration};
use crate::compatibility::CompatibilityBehavior;
use crate::websocket;
use serde::Deserialize;

#[derive(Deserialize)]
struct CommandData {
    export: String,
    arguments: Vec<serde_json::Value>,
}

pub struct Compatibility {
    command: Command,
}

#[async_trait::async_trait]
impl CompatibilityBehavior for Compatibility {
    async fn new() -> Self {
        Compatibility {
            command: Command::new(),
        }
    }

    async fn execute(
        &mut self,
        socket: &mut websocket::SocketConnection,
        data: serde_json::Value,
        id: String,
    ) {
        let parsed_data: CommandData;
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
            "command" => {
                let config: CommandConfiguration =
                    serde_json::from_value(parsed_data.arguments[0].to_owned()).unwrap();
                let result = self.command.command(config);
                socket.send(&serde_json::json!({"id": id, "result": result}).to_string())
            }
            _ => {
                println!("Export not found.")
            }
        }
    }
}
