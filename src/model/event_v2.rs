use log::info;
use lombok::{Getter, GetterMut, Setter};
use serde::{Deserialize, Serialize};

use super::group_message::GroupMessage;

#[derive(Debug)]
pub enum EventType {
    MetaEvent(MetaEvent),       // 元事件
    OtherMessage(String),            // 消息
    GroupMessage(GroupMessage), // 群消息
    PrivateMessage(String),     // 私聊消息
    UnKnown,                    // 未知
}

#[derive(Debug)]
pub enum MessageEventType {
    Group(String),   // 群消息
    Private(String), // 私聊消息
    Other(String),   // 其他消息
}

impl MessageEventType {
    pub fn get_event_type(json_str: String) -> EventType {
        let message_type: MessageType = serde_json::from_str(&json_str).unwrap();

        info!("get_event_type");

        match message_type.get_message_type().as_str() {
            "group" => EventType::GroupMessage(serde_json::from_str(&json_str).unwrap()),
            "private" => EventType::PrivateMessage(json_str),
            "other" => EventType::OtherMessage(json_str),
            _ => EventType::UnKnown,
        }
    }
}

#[derive(Deserialize, Serialize, Getter, GetterMut, Setter)]
#[serde(crate = "rocket::serde")]
pub struct MessageType {
    message_type: String,
}

#[derive(Deserialize, Serialize, Getter, GetterMut, Setter)]
#[serde(crate = "rocket::serde")]
pub struct PostType {
    post_type: String,

    #[serde(default)]
    json_str: String,
}

impl PostType {
    pub fn from_json_str(json_str: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut post_type: PostType = serde_json::from_str(&json_str)?;

        post_type.set_json_str(json_str);

        Ok(post_type)
    }
    pub fn get_enum(&self) -> EventType {
        match self.post_type.as_str() {
            "meta_event" => MetaEvent::get_event_type(self.json_str.clone()),
            "message" => MessageEventType::get_event_type(self.json_str.clone()),
            _ => EventType::UnKnown,
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

impl MetaEvent {
    pub fn get_event_type(json_str: String) -> EventType {
        EventType::MetaEvent(serde_json::from_str(&json_str).unwrap())
    }
}
