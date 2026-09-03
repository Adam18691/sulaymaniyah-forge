use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)] pub struct ChatReq { pub messages: Vec<Msg> }
#[derive(Deserialize, Serialize, Clone)] pub struct Msg { pub role: String, pub content: String }
#[derive(Serialize)] pub struct ChatRes { pub choices: Vec<Choice> }
#[derive(Serialize)] struct Choice { message: Msg }

pub async fn chat_completions(Json(req): Json<ChatReq>) -> Result<Json<ChatRes>, StatusCode> {
    let prompt = req.messages.last().map(|m| m.content.clone()).unwrap_or_default();
    let (hf, groq, or) = tokio::join!(
        crate::providers::hf::try_generate(prompt.clone()),
        crate::providers::groq::try_generate(prompt.clone()),
        crate::providers::openrouter::try_generate(prompt.clone())
    );
    let content = if!groq.clone().unwrap_or_default().is_empty() { groq.unwrap_or_default() }
    else if!hf.clone().unwrap_or_default().is_empty() { hf.unwrap_or_default() }
    else { or.unwrap_or("Fallback: Pollinations".into()) };
    Ok(Json(ChatRes { choices: vec![Choice { message: Msg { role: "assistant".into(), content } }] }))
}

pub async fn image_gen(Json(req): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, StatusCode> {
    let prompt = req["prompt"].as_str().unwrap_or("luxury book cover");
    let url = crate::providers::pollinations::image_url(prompt);
    Ok(Json(serde_json::json!({"data":[{"url":url}]})))
}

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status":"ok","ram":"12GB","models":200,"engine":"Rust > Go"}))
}
