use clap::{Parser, Subcommand};
use librarian_core::{HardwareProfile, LibrarianConfig, LibrarianDb};
use librarian_server::{create_app, ServerState};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "mazzaroth", author, version, about = "Mazzaroth — Sovereign Knowledge Galaxy & Offline AI Librarian")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, global = true, default_value = "127.0.0.1:8080")]
    bind: String,

    #[arg(short, long, global = true, default_value = "data/mazzaroth.db")]
    db: PathBuf,

    #[arg(long, global = true, default_value = "standard")]
    profile: HardwareProfile,

    #[arg(long, global = true)]
    dev_ui: Option<PathBuf>,

    #[arg(long, global = true, default_value = "content")]
    content_dir: PathBuf,

    #[arg(long, global = true, default_value = "maps")]
    maps_dir: PathBuf,

    #[arg(long, global = true, default_value = "galaxy/data/bibles")]
    bibles_dir: PathBuf,

    #[arg(long, global = true, default_value = "constellation/data")]
    constellations_dir: PathBuf,

    #[arg(long, global = true, default_value = "http://127.0.0.1:11434")]
    llm_endpoint: String,

    #[arg(long, global = true, default_value = "ministral-3:3b")]
    llm_model: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start the offline librarian server (default)
    Serve,
    /// Ingest a directory of JSON books with mandatory provenance validation
    Ingest {
        /// Directory containing JSON content files
        dir: PathBuf,
    },
    /// Run diagnostic checks on RAM, disk, SQLite FTS5 integrity, and LLM reachability
    Doctor,
    /// Optimize SQLite database and rebuild FTS5 index
    Reindex,
    /// Offline map regions (PMTiles): fetch street-level packs, list installed
    Maps {
        #[command(subcommand)]
        action: MapsCommands,
    },
}

#[derive(Subcommand, Debug)]
enum MapsCommands {
    /// Fetch a street-level map region from the Protomaps planet build (requires internet once)
    Fetch {
        /// Bounding box as MIN_LON,MIN_LAT,MAX_LON,MAX_LAT (WGS84 decimal degrees)
        bbox: String,
        /// Output region name (default: derived from bbox center, e.g. region-33.91S-18.42E)
        #[arg(short, long)]
        name: Option<String>,
        /// Max zoom: 10≈overview .. 14≈country .. 15=street detail (each +1 ≈ doubles size)
        #[arg(short, long, default_value_t = 15)]
        maxzoom: u8,
        /// Source PMTiles archive URL (default: latest Protomaps daily planet build)
        #[arg(long)]
        source: Option<String>,
    },
    /// List installed offline map regions
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Ingest { dir }) => {
            info!(db = ?cli.db, dir = ?dir, "Ingesting content into librarian database");
            let database = LibrarianDb::open(&cli.db)?;
            let count = database.ingest_directory(&dir)?;
            info!(ingested_docs = count, total_docs = database.count_documents()?, "Ingestion complete");
        }
        Some(Commands::Doctor) => {
            run_doctor(&cli.db, &cli.llm_endpoint, &cli.bind).await?;
        }
        Some(Commands::Reindex) => {
            info!(db = ?cli.db, "Optimizing database and FTS5 search index");
            let database = LibrarianDb::open(&cli.db)?;
            database.rebuild_index()?;
            info!(
                nodes = database.count_nodes()?,
                docs = database.count_documents()?,
                chunks = database.count_chunks()?,
                "Reindex and FTS5 vacuum integrity verification OK"
            );
        }
        Some(Commands::Maps { action }) => match action {
            MapsCommands::Fetch {
                bbox,
                name,
                maxzoom,
                source,
            } => {
                run_maps_fetch(&cli.maps_dir, &bbox, name, maxzoom, source).await?;
            }
            MapsCommands::List => {
                run_maps_list(&cli.maps_dir)?;
            }
        },
        Some(Commands::Serve) | None => {
            run_server(
                cli.bind,
                cli.db,
                cli.profile,
                cli.dev_ui,
                cli.content_dir,
                cli.maps_dir,
                cli.bibles_dir,
                cli.constellations_dir,
                cli.llm_endpoint,
                cli.llm_model,
            )
            .await?;
        }
    }

    Ok(())
}

async fn run_server(
    bind: String,
    db: PathBuf,
    profile: HardwareProfile,
    dev_ui: Option<PathBuf>,
    content_dir: PathBuf,
    maps_dir: PathBuf,
    bibles_dir: PathBuf,
    constellations_dir: PathBuf,
    llm_endpoint: String,
    llm_model: String,
) -> anyhow::Result<()> {
    let database = LibrarianDb::open(&db)?;

    // Auto-ingest sample content if documents empty and content dir exists
    if database.count_documents()? == 0 && content_dir.exists() {
        info!("Database empty. Auto-ingesting starter corpus from {:?}", content_dir);
        let _ = database.ingest_directory(&content_dir);
    }

    let config = LibrarianConfig {
        profile,
        db_path: db,
        content_dir,
        maps_dir,
        bibles_dir,
        constellations_dir,
        llm_endpoint,
        llm_model,
        bind_addr: bind.clone(),
    };

    let state = ServerState::new(database, config, dev_ui);
    let app = create_app(state.clone());

    let addr: SocketAddr = bind.parse()?;
    let listener = TcpListener::bind(&addr).await?;
    let local_url = format!("http://{}", addr);

    info!(url = %local_url, profile = %profile.as_str(), "📚 Mazzaroth Knowledge Sphere listening on {}", local_url);

    if profile.supports_llm() {
        if state.is_llm_reachable().await {
            info!("✦ Local Librarian AI reachable at {}", state.config.llm_endpoint);
        } else {
            info!("ℹ Local Librarian AI offline (Ollama not detected at {}). Search and reading remain fully operational.", state.config.llm_endpoint);
        }
    }

    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn run_doctor(db_path: &PathBuf, llm_endpoint: &str, bind_addr: &str) -> anyhow::Result<()> {
    println!("\n=== 📚 Mazzaroth System Diagnostics ===");

    // 1. Storage & Database Integrity Check
    if db_path.exists() {
        match LibrarianDb::open(db_path) {
            Ok(db) => {
                let nodes = db.count_nodes().unwrap_or(0);
                let docs = db.count_documents().unwrap_or(0);
                let chunks = db.count_chunks().unwrap_or(0);
                let integrity = db.check_integrity().unwrap_or(false);
                let status_icon = if integrity { "✓" } else { "✗" };
                println!("  [{}] Database:       {} ({} nodes, {} docs, {} chunks, integrity: {})", status_icon, db_path.display(), nodes, docs, chunks, if integrity { "OK" } else { "FAILED" });
            }
            Err(e) => println!("  [✗] Database:       Error opening {}: {}", db_path.display(), e),
        }
    } else {
        println!("  [!] Database:       {} (not initialized yet)", db_path.display());
    }

    // 2. Hardware RAM check
    #[cfg(target_os = "linux")]
    {
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            if let Some(total_line) = meminfo.lines().find(|l| l.starts_with("MemTotal:")) {
                if let Some(kb_str) = total_line.split_whitespace().nth(1) {
                    if let Ok(kb) = kb_str.parse::<u64>() {
                        let mb = kb / 1024;
                        let profile = if mb < 6000 { "tiny (4GB)" } else if mb < 12000 { "standard (8GB)" } else { "full (16GB+)" };
                        println!("  [✓] Memory:         {} MB total -> recommended profile: {}", mb, profile);
                    }
                }
            }
        }
    }

    // 3. Network Interface & Sovereign Exposure Check
    if bind_addr.starts_with("0.0.0.0") || bind_addr.starts_with("[::]") {
        println!("  [!] Network:        Bound to {} (LAN accessible — anyone on local network can read vault or trigger map fetches; use 127.0.0.1 for local isolation)", bind_addr);
    } else {
        println!("  [✓] Network:        Isolated to loopback {}", bind_addr);
    }

    // 4. LLM Reachability Check
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(1500))
        .build()?;

    let ollama_url = format!("{}/api/tags", llm_endpoint.trim_end_matches('/'));
    match client.get(&ollama_url).send().await {
        Ok(resp) if resp.status().is_success() => {
            println!("  [✓] Librarian AI:   Ollama online at {}", llm_endpoint);
        }
        _ => {
            println!("  [!] Librarian AI:   Offline at {} (OK: search, reading & maps remain 100% active)", llm_endpoint);
        }
    }

    println!("=======================================\n");
    Ok(())
}

// ---------------------------------------------------------------------------
// M5: offline map region packs (mazzaroth maps fetch <bbox>)
// Engine lives in librarian_server::fetch — shared with the browser downloader.
// ---------------------------------------------------------------------------

async fn run_maps_fetch(
    maps_dir: &PathBuf,
    bbox: &str,
    name: Option<String>,
    maxzoom: u8,
    source: Option<String>,
) -> anyhow::Result<()> {
    let bbox = librarian_server::fetch::parse_bbox(bbox)?;

    // live terminal progress: progress bars overwrite in place, log lines print
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    let bar_active = Arc::new(AtomicBool::new(false));
    let sink_flag = bar_active.clone();
    let progress: librarian_server::fetch::Progress = Arc::new(move |line: String| {
        if line.contains("fetching chunks") || line.contains("requesting") {
            print!("\r{line}");
            use std::io::Write;
            let _ = std::io::stdout().flush();
            sink_flag.store(true, Ordering::Relaxed);
        } else {
            if sink_flag.swap(false, Ordering::Relaxed) {
                println!();
            }
            println!("{line}");
        }
    });

    let region = librarian_server::fetch::run_fetch(
        maps_dir,
        bbox,
        name,
        maxzoom,
        source,
        progress,
    )
    .await?;
    println!("  • listed at GET /api/maps  • served at /maps/{region}.pmtiles");
    println!("  • refresh the Map view in the UI to switch regions");
    Ok(())
}

fn run_maps_list(maps_dir: &PathBuf) -> anyhow::Result<()> {
    let mut regions: Vec<(String, u64)> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(maps_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("pmtiles") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                regions.push((filename, size));
            }
        }
    }
    regions.sort_by(|a, b| a.0.cmp(&b.0));
    if regions.is_empty() {
        println!("no regions installed in {} (fetch one: mazzaroth maps fetch MIN_LON,MIN_LAT,MAX_LON,MAX_LAT)", maps_dir.display());
        return Ok(());
    }
    println!("installed map regions ({}):", regions.len());
    for (filename, size) in regions {
        println!("  {:<32} {:>10}", filename, librarian_server::fetch::human_size(size));
    }
    Ok(())
}
