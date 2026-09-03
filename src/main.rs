mod core;
mod providers;
mod book;
mod image;
mod audio;

use clap::Parser;

#[derive(Parser)]
#[command(name="forge", about="Sulaymaniyah Forge - 12GB RAM, 200 Models, Rust")]
struct Args {
    #[arg(long)] topic: Option<String>,
    #[arg(long, default_value="ar")] lang: String,
    #[arg(long)] serve: bool,
    #[arg(long, default_value="3000")] port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    if let Some(topic) = args.topic {
        println!("🚀 Forge Complete - Topic: {} | 12GB RAM | 200 Models", topic);
        let research = core::research::gather(&topic).await;
        let chapters = book::factory::write_chapters(&topic, &research, &args.lang).await?;
        let cover = image::generate_cover(&topic).await?;
        let illus = image::generate_illustrations(&chapters).await?;
        audio::generate_audiobook(&chapters).await?;
        book::factory::export_pdf(&chapters, &cover, &illus, &args.lang)?;
        println!("✅ Done: book.md + book.pdf + cover.jpg + audiobook.txt");
        return Ok(());
    }

    let app = core::server::build_router();
    let addr = format!("0.0.0.0:{}", args.port);
    println!("🔥 API http://{} - Rust faster than Go", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
