use serde::{Deserialize, Serialize};

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
