use dashmap::DashMap;
use once_cell::sync::Lazy;
pub static HEALTH: Lazy<DashMap<&'static str,bool>> = Lazy::new(|| {
    let m = DashMap::new();
    m.insert("hf", true);
    m.insert("groq", true);
    m.insert("openrouter", true);
    m.insert("pollinations", true);
    m
});
