use reqwest::Client;

use crate::{api::api::API, config::settings::SETTING};

pub async fn post(api: API) -> Result<(), Box<dyn std::error::Error>> {
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
