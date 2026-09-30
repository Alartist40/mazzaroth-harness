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
