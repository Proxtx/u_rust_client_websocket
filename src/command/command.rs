use serde::Deserialize;
use std::process;

#[derive(Deserialize)]
pub struct CommandConfiguration {
    program: String,
    args: Option<Vec<String>>,
}

pub struct Command {}

impl Command {
    pub fn new() -> Self {
        Command {}
    }

    pub fn command(&self, config: CommandConfiguration) -> String {
        let args = match config.args {
            Some(args) => args,
            None => Vec::new(),
        };

        match process::Command::new(config.program).args(args).output() {
            Ok(output) => String::from_utf8(output.stdout).unwrap(),
            Err(_) => String::from(""),
        }
    }

    pub fn command_async(&self, config: CommandConfiguration) {
        let args = match config.args {
            Some(args) => args,
            None => Vec::new(),
        };
        tokio::spawn(async move {
            match process::Command::new(config.program).args(args).spawn() {
                Ok(_) => {}
                Err(e) => {
                    println!("A command from the command_async function failed: {}", e)
                }
            }
        });
    }
}
