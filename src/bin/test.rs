use std::collections::HashMap;

fn main() {
    let json_str = r#" {
"type": 1,
"user": 2,
"raw_message": "123123",
"message": [
    {"123", "a"}
]
}
"#;

    println!("json_str = {json_str}");

    let json: HashMap<String, String> = serde_json::from_str(&json_str).unwrap();

    println!("json = {json:?}")
}
