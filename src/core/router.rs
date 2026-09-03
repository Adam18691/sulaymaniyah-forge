use anyhow::Result;

pub async fn quantum_route(prompt: &str) -> Result<String> {
    // 1- Try Groq (fastest)
    if let Ok(content) = crate::providers::groq::generate(prompt).await {
        if!content.is_empty() {
            return Ok(content);
        }
    }

    // 2- Try HuggingFace with rotation
    if let Ok(content) = crate::providers::huggingface::generate(prompt).await {
        if!content.is_empty() {
            return Ok(content);
        }
    }

    // 3- Try OpenRouter free
    if let Ok(content) = crate::providers::openrouter::generate(prompt).await {
        if!content.is_empty() {
            return Ok(content);
        }
    }

    // 4- Fallback Pollinations (unlimited free)
    let content = crate::providers::pollinations::generate(prompt).await?;
    Ok(content)
}
