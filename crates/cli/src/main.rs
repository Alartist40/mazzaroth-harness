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

    #[arg(long, global = true, default_value = "http://127.0.0.1:11434")]
    llm_endpoint: String,

    #[arg(long, global = true, default_value = "ministral-3b")]
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
            run_doctor(&cli.db, &cli.llm_endpoint).await?;
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

pub async fn run_doctor(db_path: &PathBuf, llm_endpoint: &str) -> anyhow::Result<()> {
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

    // 3. LLM Reachability Check
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
// ---------------------------------------------------------------------------

const PMTILES_MANIFEST: &str = "https://build-metadata.protomaps.dev/builds.json";
const PMTILES_BUILD_BASE: &str = "https://build.protomaps.com";

async fn run_maps_fetch(
    maps_dir: &PathBuf,
    bbox: &str,
    name: Option<String>,
    maxzoom: u8,
    source: Option<String>,
) -> anyhow::Result<()> {
    let parts: Vec<f64> = bbox
        .split(',')
        .map(|s| s.trim().parse::<f64>().map_err(|_| {
            anyhow::anyhow!("invalid bbox `{bbox}` — expected MIN_LON,MIN_LAT,MAX_LON,MAX_LAT (e.g. 18.34,-33.96,18.49,-33.86)")
        }))
        .collect::<anyhow::Result<_>>()?;
    if parts.len() != 4 {
        anyhow::bail!("bbox must have 4 comma-separated numbers, got {} — usage: mazzaroth maps fetch MIN_LON,MIN_LAT,MAX_LON,MAX_LAT", parts.len());
    }
    let (minlon, minlat, maxlon, maxlat) = (parts[0], parts[1], parts[2], parts[3]);
    if !(minlon >= -180.0 && maxlon <= 180.0 && minlat >= -90.0 && maxlat <= 90.0)
        || minlon >= maxlon
        || minlat >= maxlat
    {
        anyhow::bail!("bbox out of range or inverted: {minlon},{minlat},{maxlon},{maxlat}");
    }
    if maxzoom > 15 {
        anyhow::bail!("maxzoom {maxzoom} > 15 (PMTiles limit)");
    }

    let region_name = sanitize_region_name(name.unwrap_or_else(|| {
        let clat = (minlat + maxlat) / 2.0;
        let clon = (minlon + maxlon) / 2.0;
        format!(
            "region-{:.2}{}-{:.2}{}",
            clat.abs(),
            if clat < 0.0 { 'S' } else { 'N' },
            clon.abs(),
            if clon < 0.0 { 'W' } else { 'E' }
        )
    }));

    let source = match source {
        Some(s) => s,
        None => {
            println!("→ resolving latest Protomaps planet build ({PMTILES_MANIFEST})...");
            resolve_latest_build().await?
        }
    };
    let bin = ensure_pmtiles_bin().await?;

    std::fs::create_dir_all(maps_dir)?;
    let out_path = maps_dir.join(format!("{region_name}.pmtiles"));
    let out_str = out_path.to_string_lossy().to_string();
    let bbox_arg = format!("--bbox={minlon},{minlat},{maxlon},{maxlat}");

    println!("→ extracting [{minlon},{minlat},{maxlon},{maxlat}] z0–{maxzoom} → {}", out_path.display());
    println!("  (source: {source})");
    let status = std::process::Command::new(&bin)
        .args([
            "extract",
            &source,
            &out_str,
            &bbox_arg,
            &format!("--maxzoom={maxzoom}"),
            "--download-threads=8",
        ])
        .status()?;
    if !status.success() {
        anyhow::bail!("pmtiles extract failed (exit {status})");
    }

    let size = std::fs::metadata(&out_path)?.len();
    println!(
        "✓ region `{region_name}` ready — {} ({})",
        human_size(size),
        out_path.display()
    );
    println!("  • listed at GET /api/maps  • served at /maps/{region_name}.pmtiles");
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
        println!("  {:<32} {:>10}", filename, human_size(size));
    }
    Ok(())
}

async fn resolve_latest_build() -> anyhow::Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("mazzaroth-maps")
        .build()?;
    let builds: serde_json::Value =
        client.get(PMTILES_MANIFEST).send().await?.error_for_status()?.json().await?;
    let key = builds
        .as_array()
        .and_then(|list| list.last())
        .and_then(|b| b.get("key"))
        .and_then(|k| k.as_str())
        .ok_or_else(|| anyhow::anyhow!("unexpected {PMTILES_MANIFEST} format"))?
        .to_string();
    Ok(format!("{PMTILES_BUILD_BASE}/{key}"))
}

async fn ensure_pmtiles_bin() -> anyhow::Result<PathBuf> {
    // 1. repo-local binary (data/bin/pmtiles)
    let local = PathBuf::from("data/bin/pmtiles");
    if local.exists() {
        return Ok(local);
    }
    // 2. on PATH
    if std::process::Command::new("pmtiles")
        .arg("version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok()
    {
        return Ok(PathBuf::from("pmtiles"));
    }
    // 3. download official static binary from GitHub releases
    println!("→ pmtiles binary not found — downloading official release...");
    download_pmtiles_bin().await
}

async fn download_pmtiles_bin() -> anyhow::Result<PathBuf> {
    let client = reqwest::Client::builder()
        .user_agent("mazzaroth-maps")
        .build()?;
    let release: serde_json::Value = client
        .get("https://api.github.com/repos/protomaps/go-pmtiles/releases/latest")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let tag = release
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("v1.31.2")
        .to_string();
    let ver = tag.trim_start_matches('v');

    let os = match std::env::consts::OS {
        "linux" => "Linux",
        "macos" => "Darwin",
        other => anyhow::bail!(
            "unsupported OS `{other}` for auto-download — install pmtiles manually: https://github.com/protomaps/go-pmtiles/releases ({tag})"
        ),
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "arm64",
        other => anyhow::bail!(
            "unsupported architecture `{other}` for auto-download — install pmtiles manually: https://github.com/protomaps/go-pmtiles/releases ({tag})"
        ),
    };
    let ext = if os == "Darwin" { "zip" } else { "tar.gz" };
    let asset = format!("go-pmtiles_{ver}_{os}_{arch}.{ext}");
    let url = format!("https://github.com/protomaps/go-pmtiles/releases/download/{tag}/{asset}");

    let bin_dir = PathBuf::from("data/bin");
    std::fs::create_dir_all(&bin_dir)?;
    println!("  ↓ {url}");
    let bytes = client.get(&url).send().await?.error_for_status()?.bytes().await?;
    let archive = bin_dir.join(&asset);
    std::fs::write(&archive, &bytes)?;

    if ext == "zip" {
        let tmpdir = bin_dir.join("pmtiles-extract");
        std::fs::create_dir_all(&tmpdir)?;
        let st = std::process::Command::new("unzip")
            .args(["-o", "-q"])
            .arg(&archive)
            .arg("-d")
            .arg(&tmpdir)
            .status()?;
        if !st.success() {
            anyhow::bail!("failed to unpack {asset} (unzip) — install pmtiles manually: {url}");
        }
        let extracted = tmpdir.join("pmtiles");
        let dest = bin_dir.join("pmtiles");
        std::fs::rename(&extracted, &dest)?;
        let _ = std::fs::remove_dir_all(&tmpdir);
    } else {
        let st = std::process::Command::new("tar")
            .arg("-xzf")
            .arg(&archive)
            .arg("-C")
            .arg(&bin_dir)
            .arg("pmtiles")
            .status()?;
        if !st.success() {
            anyhow::bail!("failed to unpack {asset} (tar) — install pmtiles manually: {url}");
        }
    }
    let _ = std::fs::remove_file(&archive);

    let dest = bin_dir.join("pmtiles");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
    }
    println!("  ✓ installed {}", dest.display());
    Ok(dest)
}

fn sanitize_region_name(raw: String) -> String {
    let cleaned: String = raw
        .trim()
        .trim_end_matches(".pmtiles")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect();
    if cleaned.is_empty() {
        "region".to_string()
    } else {
        cleaned
    }
}

fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}
