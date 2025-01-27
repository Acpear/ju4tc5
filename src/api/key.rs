use axum::{body::Bytes, extract::Query, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn gen_key(Query(param): Query<GenKeyParam>) -> (StatusCode, String) {
    let uuid = match param.v7 {
        None => Uuid::new_v4(),
        Some(false) => Uuid::new_v4(),
        Some(true) => Uuid::now_v7(),
    };
    // TODO: 需要保存一下 UUID，在 has_key 里会有判别，之后再返回字符串
    (StatusCode::OK, uuid.to_string())
}

pub async fn put_key(
    Query(mut meta): Query<HashMap<String, String>>,
    file: Bytes,
) -> (StatusCode, String) {
    if let Some(key) = meta.remove("key") {
        if !has_key(key.as_str()).await {
            (
                StatusCode::NOT_FOUND,
                format!("存储定位 key={key} 不存在喵"),
            )
        } else {
            let meta = json!(meta);
            // TODO：拿着 key, meta, file 干活
            // 先做 println 玩
            println!("{key:#?}");
            println!("{meta:#?}");
            println!("{file:#?}");
            (StatusCode::OK, "成功喵".to_string())
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            "请求参数里一定要有 key 和值喵".to_string(),
        )
    }
}

pub async fn has_key(key: &str) -> bool {
    let _key = key; // 没用
    true // TODO: 开发调试，先设置 true
}

#[derive(Deserialize)]
pub struct GenKeyParam {
    v7: Option<bool>, // UUIDv7 单向增加，防止碰撞
}
