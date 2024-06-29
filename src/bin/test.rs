use coding_bot::model::event_v2::PostType;

fn main() {
    // let json_str = r#"{"self_id":3794478577,"user_id":1485513203,"time":1719649955,"message_id":-2147483557,"message_seq":-2147483557,"real_id":-2147483557,"message_type":"private","sender":{"user_id":1485513203,"nickname":"让我在摆烂一会儿","card":""},"raw_message":"二进制Debug","font":14,"sub_type":"friend","message":[{"data":{"text":"二进制Debug"},"type":"text"}],"message_format":"array","post_type":"message"}"#;
    let meta_str = r#"{"time":1719649978,"self_id":3794478577,"post_type":"meta_event","meta_event_type":"heartbeat","status":{"online":true,"good":true},"interval":30000}"#;
    // println!("json_str = {json_str}");

    let pt: PostType = serde_json::from_str(&meta_str).unwrap();

    println!("test result is {}", pt.get_post_type());

    let event = pt.get_enum(meta_str.to_string());

    println!("debug {event:?}");

    // println!("json = {json:?}")
}
