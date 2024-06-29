use serde_json::json;

fn main() {
    let group_mes = r#"{"self_id":3794478577,"user_id":1485513203,"time":1719658472,"message_id":-2147483541,"message_seq":-2147483541,"real_id":-2147483541,"message_type":"group","sender":{"user_id":1485513203,"nickname":"让我在摆烂一会儿","card":"","role":"owner"},"raw_message":"你好","font":14,"sub_type":"normal","message":[{"data":{"text":"你好"},"type":"text"}],"message_format":"array","post_type":"message","group_id":979996822}"#;

    let info = json!(group_mes);

    println!("info is {info:?}");
}
