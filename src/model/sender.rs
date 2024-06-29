use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GroupSender {
    user_id: i64,
    nickname: String,
    card: String,
    role: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PrivateSender {
    user_id: i64,
    nickname: String,
    card: String,
}
