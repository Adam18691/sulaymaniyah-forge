pub async fn try_generate(prompt: String) -> anyhow::Result<String> {
    let key = std::env::var("GROQ_API_KEY").unwrap_or_default();
    if key.is_empty() { return Ok(String::new()); }
    let client = reqwest::Client::new();
    let res = client.post("https://api.groq.com/openai/v1/chat/completions")
       .bearer_auth(key)
       .json(&serde_json::json!({"model":"llama-3.3-70b-versatile","messages":[{"role":"user","content":prompt}]}))
       .send().await?;
    let j: serde_json::Value = res.json().await?;
    Ok(j["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
}
