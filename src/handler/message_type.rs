use std::collections::HashMap;

use reqwest::Client;
use rocket::futures::TryFutureExt;
use serde_json::Value;

use crate::{
    api::api::{ApiInfo, API},
    config::settings::SETTING,
};

pub enum MessageType {
    Private(Value),
    Group(Value),
    Other(Value),
}

impl MessageType {
    pub async fn handler(value: Value) -> Result<(), Box<dyn std::error::Error>> {
        match value["message_type"].as_str() {
            Some("private") => {
                Self::Private(value).private_handler().await.unwrap();
                Ok(())
            }
            Some("group") => Self::Group(value).group_handler(),
            Some("other") => Self::Other(value).other_handler(),
            _ => {
                unreachable!()
            }
        }
    }

    pub async fn private_handler(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("private_handler executed");

        // 复读机

        if let Self::Private(value) = self {
            let user_id = value["user_id"].clone();
            let raw_message = value["raw_message"].clone();

            println!("user_id is {user_id:?}");
            println!("raw_mes is {raw_message:?}");

            let mut api = API {
                api_name: String::new(),
                api_describe: String::new(),
                api_info: ApiInfo {
                    method: String::from("POST"),
                    path: String::from("/send_private_msg"),
                    headers: HashMap::new(),
                    param_type: String::new(),
                    params: HashMap::with_capacity(2),
                },
            };

            api.api_info
                .params
                .insert(String::from("user_id"), user_id.to_string());
            api.api_info
                .params
                .insert(String::from("message"), format!("你的消息是：{}", raw_message.to_string()));

            let setting = SETTING.clone();
            let response = Client::new()
                .post(format!("{}{}", setting.service_url, api.api_info.path))
                .json(&api.api_info.params)
                .send()
                .await
                .unwrap();

            let text = response.text().await.unwrap();

            println!("text = {text}");
        }

        Ok(())
    }

    pub fn group_handler(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("group_handler executed");
        Ok(())
    }
    pub fn other_handler(self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
