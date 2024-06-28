// use std::collections::HashMap;

use coding_bot::{
    // api::api::{ApiInfo, API},
    // config::settings::Settings,
    model::event::Event,
};
use rocket::{http::Status, post, response::content, routes, serde::json::Json};

#[post("/", data = "<data>")]
async fn handle_post(data: String) -> Result<content::RawText<String>, Status> {
    println!("{data}");

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
