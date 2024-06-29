use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub enum MessageFormatType {
    Array()
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MessageFormat {
    message_format: String,
}

struct ArrayMessageNode {
    // data: 
}

pub struct ArrayMessage {

}

// pub struct OtherMessage {}