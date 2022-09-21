use tungstenite::{connect, Message};
use url::Url;

pub struct SocketConnection {
    pub socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
    pub url: String,
}

impl SocketConnection {
    pub fn new(url: &str) -> Self {
        SocketConnection {
            socket: generate_socket(url),
            url: url.to_string()
        }
    }

    pub fn send(&mut self, msg: &str) {
      self.socket.write_message(Message::Text(msg.to_string())).expect("Error sending websocket message.");
    }

    pub fn reconnect(&mut self) {
      self.socket = generate_socket(&self.url);
    }

    pub fn read_message(&mut self) -> String {
      match self.socket.read_message() {
        Ok(msg) => {
          msg.to_text().unwrap().to_string()
        }
        Err(_) => {
          self.reconnect();
          String::from("")
        }
      }
    }
}

fn generate_socket(url: &str) -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>> {
    let socket;

    loop {
        match connect(Url::parse(url).unwrap()){
            Ok((inner_socket, _response)) => {
                socket = inner_socket; 
                break
            },
            Err(_) => println!("error connecting. retry")
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    socket
}