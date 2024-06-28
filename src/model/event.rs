use std::collections::HashMap;

// use lombok::{Getter, GetterMut, Setter, ToString};
use serde::{Deserialize, Serialize};

// #[derive(Getter, GetterMut, Setter, ToString)]
// pub struct Event {
//     time: i64,
//     self_id: i64,
//     post_type: String, // message, notice, request, meta_event
// }

#[derive(Serialize, Deserialize, Debug)]
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
pub struct Anonymous {
    id: i64,      // 匿名用户 ID
    name: String, // 匿名用户名称
    flag: String, // 匿名用户 flag
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    r#type: String,
    data: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    time: Option<i64>,            // 时间戳
    self_id: Option<i64>,         // 收到事件的机器人 QQ 号
    post_type: Option<String>,    // 上报类型： message, notice, request, meta_event
    sub_type: Option<String>,     // 消息子类型： frient, group, other
    message_id: Option<i32>,      // 消息 ID
    user_id: Option<i64>,         // 发送者 QQ 号
    group_id: Option<i64>,        // 群号
    message: Option<Segment>,     // 消息内容
    raw_message: Option<String>,  // 原始消息内容
    font: Option<i32>,            // 字体
    anonymous: Option<Anonymous>, // 匿名信息
    sender: Option<Sender>,       // 发送人信息
}
