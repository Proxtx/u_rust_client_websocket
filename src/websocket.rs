use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

pub struct SocketConnection {
    sender: Sender<Message>,
}

impl SocketConnection {
    pub fn new(socket_sender: Sender<Message>) -> Self {
        SocketConnection {
            sender: socket_sender,
        }
    }

    pub async fn send(&self, msg: &str) {
        self.sender.send(Message::from(msg)).await.unwrap();
    }
}
