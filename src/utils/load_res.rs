use std::fs;

use crate::api::api::API;

pub fn load_api() -> Vec<API> {
    let content = fs::read_to_string("./resources/api/api.json").unwrap();

    let api: Vec<API> = serde_json::from_str(&content).unwrap();

    api
}

#[cfg(test)]
mod util_test {
    use super::load_api;

    #[test]
    fn load_api_test() {
        load_api();
    }
}
