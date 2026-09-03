pub async fn try_generate(prompt: String) -> anyhow::Result<String> {
    let key = std::env::var("OPENROUTER_API_KEY").unwrap_or_default();
    if key.is_empty() { return Ok(String::new()); }
    let client = reqwest::Client::new();
    let res = client.post("https://openrouter.ai/api/v1/chat/completions")
       .bearer_auth(key)
       .json(&serde_json::json!({"model":"openrouter/auto:free","messages":[{"role":"user","content":prompt}]}))
       .send().await?;
    let j: serde_json::Value = res.json().await?;
    Ok(j["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
}
