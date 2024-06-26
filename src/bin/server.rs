use std::collections::HashMap;

use coding_bot::{
    api::api::{ApiInfo, API},
    config::settings::Settings,
};
use rocket::{http::Status, post, response::content, routes};

#[post("/", data = "<data>")]
async fn handle_post(data: String) -> Result<content::RawText<String>, Status> {
    println!("res data is {}", data);

    let setting = Settings::from_config();

    let mut api = API {
        api_name: String::new(),
        api_describe: String::new(),
        api_info: ApiInfo {
            method: "POST".to_string(),
            path: "/send_private_msg".to_string(),
            param_type: "JSON".to_string(),
            headers: HashMap::new(),
            params: HashMap::new(),
        },
    };

    api.api_info
        .params
        .insert(String::from("user_id"), String::from("1485513203"));
    api.api_info.params.insert(
        String::from("message"),
        String::from("Rust 太难了，不想学了"),
    );

    // let response = Client::new()
    let response = reqwest::Client::new()
        .post(format!("{}{}", setting.service_url, api.api_info.path))
        .json(&api.api_info.params)
        .send()
        .await.unwrap();

    println!("response = {}", response.text().await.unwrap());

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
