use super::sender::PrivateSender;

pub struct PrivateMessage {
    self_id: i64,
    user_id: i64,
    time: i64,
    message_id: i32,
    message_seq: i32,
    real_id: i32,
    sender: PrivateSender,
    raw_message: String,
    font: i32,
    sub_type: String,
    // message:
    // message_format: String,
}
