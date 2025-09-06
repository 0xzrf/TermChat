use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Messages {
    Message { from: String, text: String },
    Joined { room: String },
    Error { msg: String },
    Created { room: String },
}
