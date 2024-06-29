// use std::collections::HashMap;

use coding_bot::model::{
    event::{Event, EventHandler},
    event_v2::MetaEvent,
};
use rocket::{http::Status, post, response::content, routes, serde::json::Json};

#[post("/", data = "<data>")]
async fn handle_post(data: String) -> Result<content::RawText<String>, Status> {
    println!("{data}");

    // EventHandler::MessageHandler(data.0)
    //     .handler()
    //     .await
    //     .unwrap();

    Ok(content::RawText("Received data".to_string()))
}

#[post("/", data = "<data>")]
async fn handle_meta_event(data: Json<MetaEvent>) -> Result<content::RawText<String>, Status> {
    println!("debug info : {data:?}");

    Ok(content::RawText("Recived data".to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![handle_post, handle_meta_event])
        .launch()
        .await?;

    Ok(())
}
