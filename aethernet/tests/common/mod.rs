pub fn valkey_con_str() -> String {
    std::env::var("VALKEY_URL").unwrap_or("redis://127.0.0.1/".to_string())
}
