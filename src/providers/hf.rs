pub async fn try_generate(prompt: String) -> anyhow::Result<String> {
    let token = std::env::var("HF_TOKEN").unwrap_or_default();
    if token.is_empty() { return Ok(String::new()); }
    let client = reqwest::Client::new();
    let res = client.post("https://router.huggingface.co/v1/chat/completions")
       .bearer_auth(token)
       .json(&serde_json::json!({"model":"meta-llama/Meta-Llama-3-70B-Instruct","messages":[{"role":"user","content":prompt}],"max_tokens":1024}))
       .send().await?;
    let j: serde_json::Value = res.json().await?;
    Ok(j["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
}
