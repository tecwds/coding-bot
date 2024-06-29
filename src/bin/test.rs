use coding_bot::handler::post_type::PostType;

#[tokio::main]
async fn main() {
    let group_mes = r#"{"self_id":3794478577,"user_id":1485513203,"time":1719658472,"message_id":-2147483541,"message_seq":-2147483541,"real_id":-2147483541,"message_type":"group","sender":{"user_id":1485513203,"nickname":"让我在摆烂一会儿","card":"","role":"owner"},"raw_message":"你好","font":14,"sub_type":"normal","message":[{"data":{"text":"你好"},"type":"text"}],"message_format":"array","post_type":"message","group_id":979996822}"#;
    let priv_mes = r#"{"self_id":3794478577,"user_id":1485513203,"time":1719678991,"message_id":-2147483537,"message_seq":-2147483537,"real_id":-2147483537,"message_type":"private","sender":{"user_id":1485513203,"nickname":"让我在摆烂一会儿","card":""},"raw_message":"md差点误导我，下次不能发？了","font":14,"sub_type":"friend","message":[{"data":{"text":"?"},"type":"text"}],"message_format":"array","post_type":"message"}"#;

    PostType::handler(group_mes.to_string()).await.unwrap();
    PostType::handler(priv_mes.to_string()).await.unwrap();

    // println!("info is {info:?}");
}
