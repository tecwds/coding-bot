// use std::collections::HashMap;

use coding_bot::model::event::{Event, EventHandler};
use rocket::{http::Status, post, response::content, routes, serde::json::Json};

#[post("/", data = "<data>")]
async fn handle_post(data: Json<Event>) -> Result<content::RawText<String>, Status> {
    println!("{data:?}");

    EventHandler::MessageHandler(data.0).handler();

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
