use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// # API 基本信息
///
/// 包括:
///
/// 1. API 名称
/// 2. API 描述
/// 3. API 详细结构
#[derive(Debug, Deserialize, Serialize)]
pub struct API {
    pub api_name: String,     // API 名称
    pub api_describe: String, // API 描述
    pub api_info: ApiInfo,    // API 详细结构
}


/// # API 详细结构
///
/// 主要包括：
///
/// 1. 请求方法类型
/// 2. 请求路径
/// 3. 请求头
/// 4. 请求参数
///
/// 对于**请求路径**，需要的是域名之后的资源路径；
///
/// **请求头**和**请求参数**都以Hashmap来存储，暂时为 HashMap<String,String>
///
/// Hashmap使用 rust 默认实现
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiInfo {
    pub method: String,
    pub path: String,
    pub param_type: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
}


#[cfg(test)]
mod api_test {
    // use super::*;

    #[test]
    fn test_api() {

    }
}