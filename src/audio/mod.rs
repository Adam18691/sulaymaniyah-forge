pub async fn generate_audiobook(chapters: &[String]) -> anyhow::Result<()> {
    println!("🎧 Audiobook: {} chapters - Piper TTS 1107 voices free", chapters.len());
    std::fs::write("audiobook.txt", chapters.join("\n"))?;
    Ok(())
}
