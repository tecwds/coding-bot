// use std::collections::HashMap;

use coding_bot::model::{
    event::{Event, EventHandler},
    event_v2::MetaEvent,
};
use log::info;
use rocket::{http::Status, post, response::content, routes, serde::json::Json};
use serde_json::json;

/// # 监听上报事件
///
/// 处理流程：
///
/// 1. 获取 json 字符串
/// 2. 判断 post_type 字段
/// 3. 根据 post_type 字段进行进一步处理
/// 4. 以 message 为例，后续判断 message_type
/// 5. 由特定结构体接受信息 GroupMessage、PrivateMessage、Message等
#[post("/", data = "<event>")]
async fn handle_post(event: String) -> Result<content::RawText<String>, Status> {
    info!("收到上报事件, 载荷为：{}", event);

    let event_json = json!(event);

    println!("event_json is {event_json:?}");

    Ok(content::RawText("Received data".to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![handle_post])
        .launch()
        .await?;

    Ok(())
}
