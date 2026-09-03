pub async fn generate_cover(topic: &str) -> anyhow::Result<String> {
    let url = crate::providers::pollinations::image_url(&format!("luxury book cover {}", topic));
    println!("🎨 Cover: {}", url);
    Ok(url)
}
pub async fn generate_illustrations(chapters: &[String]) -> anyhow::Result<Vec<String>> {
    let mut urls = vec![];
    for ch in chapters.iter().take(3) { urls.push(crate::providers::pollinations::image_url(ch)); }
    Ok(urls)
}
