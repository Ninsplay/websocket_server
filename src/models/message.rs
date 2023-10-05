use serde::{Deserialize, Serialize};
use warp::ws::Message as WsMessage;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub source: String,
    pub target: String,
    pub message: serde_json::Value,
}

impl Message {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let message = serde_json::from_str(json)?;
        Ok(message)
    }

    pub fn to_ws_message(&self) -> WsMessage {
        WsMessage::text(self.to_json())
    }
}
