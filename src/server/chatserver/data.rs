use serde_json;
use serde::{Serialize, Deserialize};
use websocket;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ConnectedClient {
    username: String,
    id: u64,
    // Color, Avatar, ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    sender: String,
    timestamp: u64,
    body: String
}

impl ChatMessage {
    pub fn into_message(&self) -> websocket::Message {
        let json = serde_json::to_string(&self).unwrap();
        websocket::Message::text(json)
    }
}
