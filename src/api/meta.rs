use crate::api::has_key;

// 元信息肯定要指定 key 的，所以一定要用到 has_key 的
async fn _useless() {
    has_key("编译器 不 要 警 告 我").await;
}
