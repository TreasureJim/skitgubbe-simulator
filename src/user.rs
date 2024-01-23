use axum::extract::ws::{WebSocket, Message};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};

pub struct User {
    pub id: uuid::Uuid,
    pub sender: SplitSink<WebSocket, axum::extract::ws::Message>,
    pub receiver: SplitStream<WebSocket>,
}

impl User {
    pub fn new(socket: WebSocket) -> Self {
        let (sender, receiver) = socket.split();
        Self {
            id: uuid::Uuid::new_v4(),
            sender,
            receiver,
        }
    }

    pub async fn send(&mut self, s: &str) -> Result<(), axum::Error> {
        self.sender.send(Message::Text(s.to_string())).await
    }
}
