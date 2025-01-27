use axum::extract::Query;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GenKeyParam {
    v7: Option<bool>, // UUIDv7 单向增加，防止碰撞
}

pub async fn gen_key(Query(param): Query<GenKeyParam>) -> String {
    let uuid = match param.v7 {
        None => Uuid::new_v4(),
        Some(false) => Uuid::new_v4(),
        Some(true) => Uuid::now_v7(),
    };
    uuid.to_string()
}
