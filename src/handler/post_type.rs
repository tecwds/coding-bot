use log::error;
use serde_json::Value;

use super::message_type::MessageType;

pub enum PostType {
    Notice(Value),
    Message(Value),
    MetaEvent(Value),
}

impl PostType {
    pub async fn handler(json_str: String) -> Result<(), Box<dyn std::error::Error>> {
        let value: Value = serde_json::from_str(&json_str)?;

        match value["post_type"].as_str() {
            Some("notice") => Self::Notice(value).notice_handler(),
            Some("message") => {
                Self::Message(value).message_handler().await?;
                Ok(())
            }
            Some("meta_event") => Self::MetaEvent(value).meta_handler(),
            _ => {
                error!("上报字段错误, {}", file!());
                unreachable!();
            }
        }
    }

    pub fn notice_handler(self) -> Result<(), Box<dyn std::error::Error>> {
        if let Self::Notice(value) = self {
            // TODO
            Ok(())
        } else {
            unreachable!()
        }
    }

    pub async fn message_handler(self) -> Result<(), Box<dyn std::error::Error>> {
        if let Self::Message(value) = self {
            MessageType::handler(value).await?;

            Ok(())
        } else {
            unreachable!()
        }
    }

    pub fn meta_handler(self) -> Result<(), Box<dyn std::error::Error>> {
        if let Self::MetaEvent(value) = self {
            Ok(())
        } else {
            unreachable!()
        }
    }
}
