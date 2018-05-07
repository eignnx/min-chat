extern crate serde_json;

use websocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
