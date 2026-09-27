use clap::Parser;
use mazzaroth::{create_router, CorpusImporter, MazzarothEngine, ServerState};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about = "Mazzaroth Cognitive Celestial Memory Daemon & MCP Server")]
struct Cli {
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    bind: SocketAddr,

    #[arg(short, long, default_value = "data/mazzaroth.db")]
    db: PathBuf,

    #[arg(long, default_value = "bibles")]
    bibles_dir: PathBuf,

    #[arg(long)]
    import_corpus: bool,

    #[arg(long, default_value_t = true)]
    open_browser: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "mazzaroth=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    if let Some(parent) = cli.db.parent() {
        std::fs::create_dir_all(parent)?;
    }

    info!(db_path = ?cli.db, "Opening Mazzaroth cognitive storage");
    let engine = MazzarothEngine::open(&cli.db)?;

    // If database is new or --import-corpus is passed, import the large multilingual corpus
    if (cli.import_corpus || engine.store.count_nodes()? <= 2) && cli.bibles_dir.exists() {
        info!("Seeding Mazzaroth Celestial Galaxy with multilingual corpus from {:?}", cli.bibles_dir);
        let count = CorpusImporter::import_bibles_directory(&engine, &cli.bibles_dir, 60, 10)?;
        info!(imported_nodes = count, "Corpus seeded into Mazzaroth Galaxy");
    }

    // Continuous background physics simulation thread
    let sim_engine = engine.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(50));
        loop {
            interval.tick().await;
            let _ = sim_engine.step_physics(0.05);
        }
    });

    let state = ServerState { engine };
    let router = create_router(state);

    let listener = TcpListener::bind(&cli.bind).await?;
    let local_url = format!("http://127.0.0.1:{}", cli.bind.port());
    info!(bind = %cli.bind, url = %local_url, "🌌 Mazzaroth Cognitive Galaxy listening. Open {} in your browser.", local_url);

    if cli.open_browser {
        let url_to_open = local_url.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
            let _ = std::process::Command::new("xdg-open")
                .arg(&url_to_open)
                .spawn();
        });
    }

    axum::serve(listener, router).await?;
    Ok(())
}
