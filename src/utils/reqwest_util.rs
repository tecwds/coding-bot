use std::collections::HashMap;

use log::info;
use reqwest::Client;

use crate::{api::api::API, config::settings::SETTING};

fn check_param(api: &API, params: &HashMap<String, String>) -> bool {
    let param_detail = &api.api_info.params;

    let api_iter = param_detail.iter();
    let par_iter= params.iter();

    if param_detail.len() != params.len() {
        info!("{} - 参数数量不匹配", api.api_describe);
    }



    false

}

pub async fn post(api: API, headers: Option<HashMap<String,String>>, params: HashMap<String,String>) -> Result<(), Box<dyn std::error::Error>> {
    let host = SETTING.service_url.clone();

    let api_url = format!("{host}{}", api.api_info.path);

    let client = Client::new();

    let res = client
        .post(api_url)
        .json(&api.api_info.params)
        .send()
        .await?;

    let text = res.text().await?;
    // let text = client
    // let text = client.
    // let text = client.text
    Ok(())
}
