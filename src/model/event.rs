use std::{collections::HashMap, hash::Hash};

use log::info;
use reqwest::Client;
// use lombok::{Getter, GetterMut, Setter, ToString};
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::{
    api::{
        self,
        api::{ApiInfo, API},
    },
    config::settings::{self, SETTING},
};

// #[derive(Getter, GetterMut, Setter, ToString)]
// pub struct Event {
//     time: i64,
//     self_id: i64,
//     post_type: String, // message, notice, request, meta_event
// }
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Sender {
    user_id: Option<String>,  // 发送者 QQ
    nickname: Option<String>, // 昵称
    sex: Option<String>,      // 性别 male, female, unknown
    age: Option<i32>,         // 年龄
    card: Option<String>,     // 群名片/备注
    area: Option<String>,     // 地区
    level: Option<String>,    // 成员等级
    role: Option<String>,     // 角色 owner, admin, member
    title: Option<String>,    // 专属头衔
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Anonymous {
    id: i64,      // 匿名用户 ID
    name: String, // 匿名用户名称
    flag: String, // 匿名用户 flag
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    r#type: String,
    data: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Segment {
    Text(Message),
    Face(Message),
    Image(Message),
    Record(Message),
    Video(Message),
    At(Message),
    Rps(Message),
    Dice(Message),
    Shake(Message),
    Poke(Message),
    Anonymous(Message),
    Share(Message),
    Contact(Message),
    Location(Message),
    Music(Message),
    Reply(Message),
    Forward(Message),
    Node(Message),
    Xml(Message),
    Json(Message),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Status {
    online: Option<bool>,
    good: Option<bool>,
    interval: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Event {
    time: Option<i64>,               // 时间戳
    self_id: Option<i64>,            // 收到事件的机器人 QQ 号
    post_type: Option<String>,       // 上报类型： message, notice, request, meta_event
    sub_type: Option<String>,        // 消息子类型： frient, group, other
    meta_event_type: Option<String>, // 元事件类型
    status: Option<Status>,          // 状态
    message_id: Option<i32>,         // 消息 ID
    user_id: Option<String>,         // 发送者 QQ 号
    real_id: Option<String>,         // 真实 ID -> message_id
    group_id: Option<String>,        // 群号
    message: Option<String>,         // 消息内容
    message_seq: Option<i32>,        // 消息 ID ?
    message_type: Option<String>,    // 消息类型
    message_format: Option<String>,  // 消息格式化类型
    raw_message: Option<String>,     // 原始消息内容
    font: Option<i32>,               // 字体
    anonymous: Option<Anonymous>,    // 匿名信息
    sender: Option<Sender>,          // 发送人信息
}

/// 事件处理器
pub enum EventHandler {
    MessageHandler(Event),
}

impl EventHandler {
    pub async fn handler(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            EventHandler::MessageHandler(event) => {
                let settings = SETTING.clone();
                // 判断消息类型 message_type
                if let Some(msg_type) = event.message_type.clone() {
                    if msg_type == String::from("private") {
                        info!("msg_type 为 private");

                        let mut api = API {
                            api_name: String::from("返回消息"),
                            api_describe: String::new(),
                            api_info: ApiInfo {
                                method: "POST".to_string(),
                                path: "/send_private_msg".to_string(),
                                headers: HashMap::new(),
                                param_type: "JSON".to_string(),
                                params: HashMap::new(),
                            },
                        };

                        api.api_info.params.insert(
                            String::from("user_id"),
                            String::from(event.user_id.clone().unwrap()),
                        );
                        api.api_info.params.insert(
                            String::from("message"),
                            String::from(event.raw_message.clone().unwrap().clone()),
                        );

                        // let url =
                        let response = Client::new()
                            .post(format!("{}{}", settings.service_url, api.api_info.path))
                            .json(&api.api_info.params)
                            .send()
                            .await?;

                        println!("response = {}", response.text().await?);
                    }
                }
            }
        }

        Ok(())
    }
}
