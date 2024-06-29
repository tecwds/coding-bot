// use std::collections::HashMap;

use coding_bot::model::{
    event::{Event, EventHandler},
    event_v2::MetaEvent,
};
use log::info;
use rocket::{http::Status, post, response::content, routes, serde::json::Json};

#[post("/", data = "<event>")]
async fn handle_post(event: String) -> Result<content::RawText<String>, Status> {
    info!("收到上报事件");

    info!("event = {event}");
    // EventHandler::MessageHandler(data.0)
    //     .handler()
    //     .await
    //     .unwrap();

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
