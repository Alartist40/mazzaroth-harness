use clap::Parser;
use mazzaroth::{create_router, MazzarothEngine, ServerState};
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
    info!(bind = %cli.bind, "🌌 Mazzaroth Cognitive Galaxy listening. Open http://localhost:8080 to view the 3D universe.");

    axum::serve(listener, router).await?;
    Ok(())
}
