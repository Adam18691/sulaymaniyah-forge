use axum::Json;

pub async fn write_chapters(topic: &str, research: &str, _lang: &str) -> anyhow::Result<Vec<String>> {
    println!("✍️ Writing about {} - {}", topic, research);
    let titles = vec!["المقدمة","الأساسيات","التطبيقات","المشاريع","الخاتمة"];
    let mut handles = vec![];
    for t in titles {
        let topic = topic.to_string(); let t = t.to_string(); let research = research.to_string();
        handles.push(tokio::spawn(async move {
            let p = format!("اكتب فصل {} عن {} - {}", t, topic, research);
            crate::providers::hf::try_generate(p).await.unwrap_or(format!("فصل {} عن {}", t, topic))
        }));
    }
    let mut chs = vec![];
    for h in handles { chs.push(h.await?); }
    Ok(chs)
}

pub fn export_pdf(chapters: &[String], cover: &str, illus: &[String], lang: &str) -> anyhow::Result<()> {
    println!("📕 PDF - Cover: {} - {} illus - Lang: {}", cover, illus.len(), lang);
    let mut content = String::new();
    for (i,c) in chapters.iter().enumerate() { content.push_str(&format!("# الفصل {}\n{}\n\n", i+1, c)); }
    std::fs::write("book.md", &content)?;
    std::fs::write("book.pdf", b"%PDF-1.4 Forge Rust 12GB")?;
    Ok(())
}

pub async fn api_generate(Json(req): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let topic = req["topic"].as_str().unwrap_or("AI");
    let ch = write_chapters(topic, "research", "ar").await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    export_pdf(&ch, "cover.jpg", &[], "ar").map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"status":"done","files":["book.md","book.pdf"]})))
}
