use crate::fetch::{self, FetchRequest, FetchStatus};
use crate::state::ServerState;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::header::{ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, RANGE};
use axum::http::{HeaderMap, Response, StatusCode};
use axum::response::Json;
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Serialize)]
pub struct MapRegion {
    pub name: String,
    pub filename: String,
    pub size_bytes: u64,
}

pub async fn handle_list_maps(State(state): State<ServerState>) -> Json<Vec<MapRegion>> {
    let mut list = Vec::new();
    let dir = &state.config.maps_dir;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("pmtiles") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
                list.push(MapRegion {
                    name,
                    filename,
                    size_bytes,
                });
            }
        }
    }
    list.sort_by(|a, b| a.filename.cmp(&b.filename));
    Json(list)
}

pub async fn handle_serve_pmtiles(
    State(state): State<ServerState>,
    Path(filename): Path<String>,
    headers: HeaderMap,
) -> Result<Response<Body>, StatusCode> {
    // Sanitize filename to prevent directory traversal
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let file_path = state.config.maps_dir.join(&filename);
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut file = File::open(&file_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let total_len = file
        .metadata()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .len();

    // Check for HTTP Range header
    if let Some(range_header) = headers.get(RANGE) {
        if let Ok(range_str) = range_header.to_str() {
            if let Some(range_val) = range_str.strip_prefix("bytes=") {
                let parts: Vec<&str> = range_val.split('-').collect();
                let start: u64 = parts[0].parse().unwrap_or(0);
                let end: u64 = if parts.len() > 1 && !parts[1].is_empty() {
                    parts[1].parse().unwrap_or(total_len - 1)
                } else {
                    total_len - 1
                };

                let end = end.min(total_len - 1);
                if start > end || start >= total_len {
                    return Ok(Response::builder()
                        .status(StatusCode::RANGE_NOT_SATISFIABLE)
                        .header(CONTENT_RANGE, format!("bytes */{}", total_len))
                        .body(Body::empty())
                        .unwrap());
                }

                let length = (end - start + 1) as usize;
                let mut buffer = vec![0u8; length];
                file.seek(SeekFrom::Start(start))
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                file.read_exact(&mut buffer)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                return Ok(Response::builder()
                    .status(StatusCode::PARTIAL_CONTENT)
                    .header(ACCEPT_RANGES, "bytes")
                    .header(CONTENT_TYPE, "application/octet-stream")
                    .header(CONTENT_LENGTH, length.to_string())
                    .header(
                        CONTENT_RANGE,
                        format!("bytes {}-{}/{}", start, end, total_len),
                    )
                    .body(Body::from(buffer))
                    .unwrap());
            }
        }
    }

    // Full file response
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(ACCEPT_RANGES, "bytes")
        .header(CONTENT_TYPE, "application/octet-stream")
        .header(CONTENT_LENGTH, total_len.to_string())
        .body(Body::from(buffer))
        .unwrap())
}

// ---------------------------------------------------------------------------
// Browser downloader (M5): one background pack at a time + status + delete
// ---------------------------------------------------------------------------

pub async fn handle_fetch_start(
    State(state): State<ServerState>,
    axum::Json(req): axum::Json<FetchRequest>,
) -> Result<Json<FetchStatus>, (StatusCode, String)> {
    let bbox = fetch::validate_bbox(&req.bbox).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let maxzoom = req.maxzoom.unwrap_or(13);
    if maxzoom > 15 {
        return Err((StatusCode::BAD_REQUEST, "maxzoom must be 15 or lower".into()));
    }
    let region = fetch::sanitize_region_name(
        req.name.unwrap_or_else(|| fetch::derive_region_name(&bbox)),
    );
    fetch::begin(&region).map_err(|m| (StatusCode::CONFLICT, m))?;

    let maps_dir = state.config.maps_dir.clone();
    tokio::spawn(async move {
        let sink = fetch::log_sink();
        let result = fetch::run_fetch(&maps_dir, bbox, Some(region), maxzoom, req.source, sink).await;
        fetch::finish(result.map_err(|e| e.to_string()));
    });

    Ok(Json(fetch::fetch_status()))
}

pub async fn handle_fetch_status() -> Json<FetchStatus> {
    Json(fetch::fetch_status())
}

pub async fn handle_delete_map(
    State(state): State<ServerState>,
    Path(filename): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err((StatusCode::BAD_REQUEST, "invalid filename".into()));
    }
    if filename == fetch::BASE_REGION {
        return Err((
            StatusCode::BAD_REQUEST,
            "the built-in world overview is protected and cannot be deleted".into(),
        ));
    }
    if !filename.ends_with(".pmtiles") {
        return Err((StatusCode::BAD_REQUEST, "not a .pmtiles region".into()));
    }
    if fetch::fetch_status().state == "running" {
        return Err((StatusCode::CONFLICT, "a download is running — wait for it to finish".into()));
    }
    let path = state.config.maps_dir.join(&filename);
    if !path.exists() {
        return Err((StatusCode::NOT_FOUND, "region not found".into()));
    }
    std::fs::remove_file(&path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
