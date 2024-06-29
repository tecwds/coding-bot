use lombok::{Getter, GetterMut, Setter};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Type {
    MetaEvent(MetaEvent), // 元事件
    Message(String),   // 消息
    UnKnown,
}

#[derive(Deserialize, Serialize, Getter, GetterMut, Setter)]
#[serde(crate = "rocket::serde")]
pub struct PostType {
    post_type: String,
}

impl PostType {
    pub fn get_enum(&self, json_str: String) -> Type {
        match self.post_type.as_str() {
            "meta_event" => Type::MetaEvent(serde_json::from_str(&json_str).unwrap()),
            "message" => Type::Message(json_str),
            _ => Type::UnKnown,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MetaStatus {
    online: bool,
    good: bool,
    interval: i32,
}

/// meta_event
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MetaEvent {
    time: i64,
    self_id: i64,
    post_type: String,
    meta_event_type: String,
    status: MetaStatus,
}
