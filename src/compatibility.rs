use crate::websocket;

#[async_trait::async_trait]
pub trait CompatibilityBehavior: Send + Sync {
    async fn new() -> Self
    where
        Self: Sized;

    async fn execute(
        &mut self,
        socket: &mut websocket::SocketConnection,
        data: serde_json::Value,
        id: String,
    );
}
