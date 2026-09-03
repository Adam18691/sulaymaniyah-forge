use tokio::sync::Mutex;
use std::collections::VecDeque;
use once_cell::sync::Lazy;
pub static ROTATOR: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));
pub async fn next_token() -> String {
    ROTATOR.lock().await.pop_front().unwrap_or_else(|| std::env::var("HF_TOKEN").unwrap_or_default())
}
