use axum::extract::Query;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CheckCollisionParam {
    check_collision: Option<bool>,
}

pub async fn gen_key(Query(param): Query<CheckCollisionParam>) -> String {
    let uuid = match param.check_collision {
        None => Uuid::new_v4(),
        Some(false) => Uuid::new_v4(),
        Some(true) => Uuid::now_v7(),
    };
    uuid.to_string()
}
