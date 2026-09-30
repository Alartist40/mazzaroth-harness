//! Shared region-pack fetch engine (M5): resolves the latest Protomaps planet
//! build, ensures the official `pmtiles` binary, and range-extracts a bbox into
//! `maps/<name>.pmtiles`. Used by the CLI (`mazzaroth maps fetch`) and the
//! browser downloader (`POST /api/maps/fetch`).

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};

pub const PMTILES_MANIFEST: &str = "https://build-metadata.protomaps.dev/builds.json";
pub const PMTILES_BUILD_BASE: &str = "https://build.protomaps.com";
pub const BASE_REGION: &str = "world.pmtiles";

/// Progress sink: every meaningful line of the fetch pipeline (resolve, binary
/// download, extract log segments, completion) is pushed here.
pub type Progress = Arc<dyn Fn(String) + Send + Sync>;

// ---------------------------------------------------------------------------
// Input validation & naming
// ---------------------------------------------------------------------------

/// Parse `MIN_LON,MIN_LAT,MAX_LON,MAX_LAT` (CLI form).
pub fn parse_bbox(raw: &str) -> anyhow::Result<[f64; 4]> {
    let parts: Vec<f64> = raw
        .split(',')
        .map(|s| s.trim().parse::<f64>().map_err(|_| {
            anyhow::anyhow!(
                "invalid bbox `{raw}` — expected MIN_LON,MIN_LAT,MAX_LON,MAX_LAT (e.g. 122.9,24.0,145.9,45.6)"
            )
        }))
        .collect::<anyhow::Result<_>>()?;
    validate_bbox(&parts)
}

/// Validate a bbox in either vector form (`[w,s,e,n]`) or flat form.
pub fn validate_bbox(parts: &[f64]) -> anyhow::Result<[f64; 4]> {
    if parts.len() != 4 {
        anyhow::bail!(
            "bbox must have 4 numbers [min_lon, min_lat, max_lon, max_lat], got {}",
            parts.len()
        );
    }
    let (minlon, minlat, maxlon, maxlat) = (parts[0], parts[1], parts[2], parts[3]);
    if !(minlon >= -180.0 && maxlon <= 180.0 && minlat >= -90.0 && maxlat <= 90.0)
        || minlon >= maxlon
        || minlat >= maxlat
    {
        anyhow::bail!("bbox out of range or inverted: {minlon},{minlat},{maxlon},{maxlat}");
    }
    Ok([minlon, minlat, maxlon, maxlat])
}

/// Default region name derived from the bbox center (e.g. `region-33.91S-18.42E`).
pub fn derive_region_name(bbox: &[f64; 4]) -> String {
    let clat = (bbox[1] + bbox[3]) / 2.0;
    let clon = (bbox[0] + bbox[2]) / 2.0;
    format!(
        "region-{:.2}{}-{:.2}{}",
        clat.abs(),
        if clat < 0.0 { 'S' } else { 'N' },
        clon.abs(),
        if clon < 0.0 { 'W' } else { 'E' }
    )
}

/// Keep region names filename-safe (used by CLI, API and delete route).
pub fn sanitize_region_name(raw: impl Into<String>) -> String {
    let cleaned: String = raw
        .into()
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

pub fn human_size(bytes: u64) -> String {
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

// ---------------------------------------------------------------------------
// Source resolution + pmtiles binary bootstrap
// ---------------------------------------------------------------------------

pub async fn resolve_latest_build() -> anyhow::Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("mazzaroth-maps")
        .build()?;
    let builds: serde_json::Value = client
        .get(PMTILES_MANIFEST)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let key = builds
        .as_array()
        .and_then(|list| list.last())
        .and_then(|b| b.get("key"))
        .and_then(|k| k.as_str())
        .ok_or_else(|| anyhow::anyhow!("unexpected {PMTILES_MANIFEST} format"))?
        .to_string();
    Ok(format!("{PMTILES_BUILD_BASE}/{key}"))
}

static BIN_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

pub async fn ensure_pmtiles_bin(progress: &Progress) -> anyhow::Result<PathBuf> {
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
    // 3. download official static binary from GitHub releases (serialized)
    let _guard = BIN_LOCK.lock().await;
    // double-check: another caller may have installed it while we waited
    if local.exists() {
        return Ok(local);
    }
    download_pmtiles_bin(progress).await
}

async fn download_pmtiles_bin(progress: &Progress) -> anyhow::Result<PathBuf> {
    progress("→ pmtiles binary not found — downloading official release...".to_string());
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
    progress(format!("  ↓ {url}"));
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
    progress(format!("  ✓ installed {}", dest.display()));
    Ok(dest)
}

// ---------------------------------------------------------------------------
// Fetch pipeline
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FetchRequest {
    /// Bounding box `[min_lon, min_lat, max_lon, max_lat]`.
    pub bbox: Vec<f64>,
    pub name: Option<String>,
    pub maxzoom: Option<u8>,
    pub source: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FetchStatus {
    /// `idle` | `running` | `done` | `error`
    pub state: String,
    /// Region name the current/last job targets.
    pub region: String,
    /// Human one-liner for the UI.
    pub message: String,
    /// Tail of the job log (last lines).
    pub log: Vec<String>,
}

impl Default for FetchStatus {
    fn default() -> Self {
        Self {
            state: "idle".into(),
            region: String::new(),
            message: String::new(),
            log: Vec::new(),
        }
    }
}

static STATUS: OnceLock<Mutex<FetchStatus>> = OnceLock::new();

fn status_cell() -> &'static Mutex<FetchStatus> {
    STATUS.get_or_init(|| Mutex::new(FetchStatus::default()))
}

pub fn fetch_status() -> FetchStatus {
    status_cell()
        .lock()
        .map(|s| s.clone())
        .unwrap_or_default()
}

/// Claim the single download slot for `region`. Err(message) when busy.
pub fn begin(region: &str) -> Result<(), String> {
    let mut s = status_cell().lock().map_err(|_| "status lock poisoned".to_string())?;
    if s.state == "running" {
        return Err(format!(
            "a download is already running: {}",
            if s.region.is_empty() { "another region".into() } else { s.region.clone() }
        ));
    }
    s.state = "running".into();
    s.region = region.to_string();
    s.message = format!("downloading {region}…");
    s.log.clear();
    s.log.push(format!("→ starting fetch for `{region}`"));
    Ok(())
}

/// Progress sink that feeds the shared job log (used by the HTTP API).
pub fn log_sink() -> Progress {
    Arc::new(push_line)
}

fn push_line(line: String) {
    let line = line.trim_end().to_string();
    if line.is_empty() {
        return;
    }
    let mut s = match status_cell().lock() {
        Ok(s) => s,
        Err(_) => return,
    };
    // collapse in-place progress-bar updates into a single tail line
    if line.contains("fetching chunks") || line.contains("requesting") {
        if let Some(last) = s.log.last_mut() {
            if last.contains("fetching chunks") || last.contains("requesting") {
                *last = line;
                return;
            }
        }
    }
    s.log.push(line);
    const MAX_LOG: usize = 60;
    if s.log.len() > MAX_LOG {
        let drain = s.log.len() - MAX_LOG;
        s.log.drain(..drain);
    }
}

/// Mark the running job as finished (ok or error). Terminal states only.
pub fn finish(result: Result<String, String>) {
    let mut s = match status_cell().lock() {
        Ok(s) => s,
        Err(_) => return,
    };
    if s.state != "running" {
        return;
    }
    match result {
        Ok(region) => {
            s.state = "done".into();
            s.region = region.clone();
            s.message = format!("{region} ready");
            s.log.push(format!("✓ `{region}` ready — listed in INSTALLED REGIONS"));
        }
        Err(e) => {
            s.state = "error".into();
            s.message = e.clone();
            s.log.push(format!("✗ {e}"));
        }
    }
}

/// Full pipeline: resolve source → ensure binary → extract (offline-blocking
/// inside a blocking task) → atomic rename into place. Returns the region name.
pub async fn run_fetch(
    maps_dir: &Path,
    bbox: [f64; 4],
    name: Option<String>,
    maxzoom: u8,
    source: Option<String>,
    progress: Progress,
) -> anyhow::Result<String> {
    let (minlon, minlat, maxlon, maxlat) = (bbox[0], bbox[1], bbox[2], bbox[3]);
    if maxzoom > 15 {
        anyhow::bail!("maxzoom {maxzoom} > 15 (PMTiles limit)");
    }
    let region = sanitize_region_name(name.unwrap_or_else(|| derive_region_name(&bbox)));

    let source = match source {
        Some(s) => s,
        None => {
            progress(format!("→ resolving latest Protomaps planet build ({PMTILES_MANIFEST})…"));
            let s = resolve_latest_build().await?;
            progress(format!("  ✓ source: {s}"));
            s
        }
    };
    let bin = ensure_pmtiles_bin(&progress).await?;

    std::fs::create_dir_all(maps_dir)?;
    let out_path = maps_dir.join(format!("{region}.pmtiles"));
    // extract to a .tmp sidecar so half-finished files never show up in the UI
    let tmp_path = maps_dir.join(format!("{region}.pmtiles.tmp"));
    if tmp_path.exists() {
        let _ = std::fs::remove_file(&tmp_path);
    }

    progress(format!(
        "→ extracting [{minlon},{minlat},{maxlon},{maxlat}] z0–{maxzoom} → {}",
        out_path.display()
    ));

    let tmp_str = tmp_path.to_string_lossy().to_string();
    let bbox_arg = format!("--bbox={minlon},{minlat},{maxlon},{maxlat}");
    let p = progress.clone();
    tokio::task::spawn_blocking(move || extract_blocking(&bin, &source, &tmp_str, &bbox_arg, maxzoom, p))
        .await
        .map_err(|e| anyhow::anyhow!("extract task failed: {e}"))??;

    std::fs::rename(&tmp_path, &out_path)?;

    let size = std::fs::metadata(&out_path)?.len();
    progress(format!(
        "✓ region `{region}` ready — {} ({})",
        human_size(size),
        out_path.display()
    ));
    Ok(region)
}

fn extract_blocking(
    bin: &Path,
    source: &str,
    out: &str,
    bbox_arg: &str,
    maxzoom: u8,
    progress: Progress,
) -> anyhow::Result<()> {
    let mut child = Command::new(bin)
        .args([
            "extract",
            source,
            out,
            bbox_arg,
            &format!("--maxzoom={maxzoom}"),
            "--download-threads=16",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let t_out = pump(
        child.stdout.take().ok_or_else(|| anyhow::anyhow!("no stdout"))?,
        progress.clone(),
    );
    let t_err = pump(
        child.stderr.take().ok_or_else(|| anyhow::anyhow!("no stderr"))?,
        progress.clone(),
    );
    let status = child.wait()?;
    let _ = t_out.join();
    let _ = t_err.join();
    if !status.success() {
        anyhow::bail!("pmtiles extract failed (exit {status})");
    }
    Ok(())
}

/// Read a child stream, split on newlines AND carriage returns (pmtiles uses
/// `\r` progress bars), forward each segment to the progress sink.
fn pump<R: Read + Send + 'static>(mut reader: R, progress: Progress) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut buf: Vec<u8> = Vec::new();
        let mut tmp = [0u8; 4096];
        loop {
            let n = match reader.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            buf.extend_from_slice(&tmp[..n]);
            let mut start = 0usize;
            for i in 0..buf.len() {
                if buf[i] == b'\n' || buf[i] == b'\r' {
                    if i > start {
                        if let Ok(seg) = std::str::from_utf8(&buf[start..i]) {
                            let seg = seg.trim_end();
                            if !seg.trim().is_empty() {
                                progress(seg.to_string());
                            }
                        }
                    }
                    start = i + 1;
                }
            }
            if start > 0 {
                buf.drain(..start);
            }
            if buf.len() > 128 * 1024 {
                buf.clear();
            }
        }
        if !buf.is_empty() {
            if let Ok(seg) = std::str::from_utf8(&buf) {
                let seg = seg.trim();
                if !seg.is_empty() {
                    progress(seg.to_string());
                }
            }
        }
    })
}
