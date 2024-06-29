use serde::{Deserialize, Serialize};

use super::{event_v2::EventType, sender::GroupSender};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GroupMessage {
    self_id: i64,
    user_id: i64,
    time: u64,
    message_id: i32,
    message_seq: i32,
    real_id: i32,
    sender: GroupSender, // TODO
    raw_message: String,
    font: i32,
    sub_type: String,
    // message: String
    message_format: String,
    group_id: i64,
}

impl GroupMessage {
    pub fn from_json_str(json_str: String) -> Self {
        serde_json::from_str(&json_str).unwrap()    
    }
}
