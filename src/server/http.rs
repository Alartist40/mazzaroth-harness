use crate::cognitive::MemoryTier;
use crate::engine::MazzarothEngine;
use crate::galaxy::ScriptureReader;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, Deserialize)]
pub struct IngestRequest {
    pub label: String,
    pub content: String,
    pub tier: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RecallQuery {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct NodeQuery {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ScriptureMetaQuery {
    pub lang: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScriptureChapterQuery {
    pub lang: Option<String>,
    pub version: Option<String>,
    pub book: Option<String>,
    pub chapter: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ScriptureVersionsQuery {
    pub lang: Option<String>,
}

#[derive(Clone)]
pub struct ServerState {
    pub engine: MazzarothEngine,
    pub scripture: ScriptureReader,
}

impl ServerState {
    pub fn new(engine: MazzarothEngine) -> Self {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let bibles_dir = PathBuf::from(manifest_dir).join("galaxy/data/bibles");
        let dir = if bibles_dir.exists() {
            bibles_dir
        } else {
            PathBuf::from("galaxy/data/bibles")
        };
        Self {
            engine,
            scripture: ScriptureReader::new(dir),
        }
    }

    pub fn with_bibles_dir(engine: MazzarothEngine, bibles_dir: PathBuf) -> Self {
        Self {
            engine,
            scripture: ScriptureReader::new(bibles_dir),
        }
    }
}

fn resolve_web_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = PathBuf::from(manifest_dir).join("web");
    if p.exists() {
        p
    } else {
        PathBuf::from("web")
    }
}

pub fn create_router(state: ServerState) -> Router {
    let web_dir = resolve_web_dir();
    let index_file = web_dir.join("index.html");

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/sections", get(handle_sections))
        .route("/api/sections/constellations", get(crate::constellation::handle_constellations_section))
        .route("/api/memory/ingest", post(handle_ingest))
        .route("/api/memory/recall", get(handle_recall))
        .route("/api/memory/node", get(handle_get_single_node))
        .route("/api/memory/celestial", get(handle_celestial))
        .route("/api/memory/nodes", get(handle_all_nodes))
        .route("/api/scripture/meta", get(handle_scripture_meta))
        .route("/api/scripture", get(handle_scripture_chapter))
        .route("/api/scripture/languages", get(handle_scripture_languages))
        .route("/api/scripture/versions", get(handle_scripture_versions))
        .route("/api/mcp", post(crate::server::mcp::handle_mcp_post))
        .fallback_service(ServeDir::new(&web_dir).fallback(ServeFile::new(index_file)))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn handle_ingest(
    State(state): State<ServerState>,
    Json(req): Json<IngestRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tier = req
        .tier
        .as_deref()
        .and_then(|s| s.parse::<MemoryTier>().ok())
        .unwrap_or(MemoryTier::Episodic);

    let tags = req.tags.unwrap_or_default();

    match state.engine.ingest(&req.label, &req.content, tier, tags) {
        Ok(node) => Ok(Json(serde_json::to_value(node).unwrap())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_recall(
    State(state): State<ServerState>,
    Query(query): Query<RecallQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let limit = query.limit.unwrap_or(10);
    match state.engine.recall(&query.q, limit) {
        Ok(nodes) => Ok(Json(serde_json::to_value(nodes).unwrap())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_get_single_node(
    State(state): State<ServerState>,
    Query(query): Query<NodeQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.engine.store.get_node(&query.id) {
        Ok(Some(node)) => {
            let links = state.engine.store.get_links_for_node(&query.id).unwrap_or_default();
            Ok(Json(json!({
                "id": node.id,
                "label": node.label,
                "content": node.content,
                "tags": node.tags,
                "tier": node.tier,
                "strength": node.strength,
                "activation": node.activation,
                "access_count": node.access_count,
                "created_at": node.created_at,
                "last_accessed": node.last_accessed,
                "pos_x": node.pos_x,
                "pos_y": node.pos_y,
                "pos_z": node.pos_z,
                "links": links,
            })))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_celestial(State(state): State<ServerState>) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.engine.get_galaxy_state() {
        Ok(galaxy) => Ok(Json(serde_json::to_value(galaxy).unwrap())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_all_nodes(State(state): State<ServerState>) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.engine.store.get_all_nodes() {
        Ok(nodes) => Ok(Json(serde_json::to_value(nodes).unwrap())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_sections() -> Json<serde_json::Value> {
    Json(json!([
        {
            "id": "galaxy",
            "label": "Celestial Galaxy",
            "api": "/api/memory/celestial",
            "description": "Multilingual scriptural superclusters & neural memory spiral"
        },
        {
            "id": "constellations",
            "label": "Classical Constellations",
            "api": "/api/sections/constellations",
            "description": "Astronomical asterisms and anchor stellar geometries"
        }
    ]))
}

async fn handle_scripture_meta(
    State(state): State<ServerState>,
    Query(query): Query<ScriptureMetaQuery>,
) -> Result<Json<crate::galaxy::ScriptureMetaResponse>, StatusCode> {
    let lang = query.lang.as_deref().unwrap_or("eng");
    let version = query.version.as_deref().unwrap_or("kjv");

    state
        .scripture
        .get_meta(lang, version)
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn handle_scripture_chapter(
    State(state): State<ServerState>,
    Query(query): Query<ScriptureChapterQuery>,
) -> Result<Json<crate::galaxy::ScriptureChapterResponse>, StatusCode> {
    let lang = query.lang.as_deref().unwrap_or("eng");
    let version = query.version.as_deref().unwrap_or("kjv");
    let book = query.book.as_deref().unwrap_or("Genesis");
    let chapter = query.chapter.unwrap_or(1);

    state
        .scripture
        .get_chapter(lang, version, book, chapter)
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn handle_scripture_languages(
    State(state): State<ServerState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    state
        .scripture
        .list_languages()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn handle_scripture_versions(
    State(state): State<ServerState>,
    Query(query): Query<ScriptureVersionsQuery>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let lang = query.lang.as_deref().unwrap_or("eng");
    state
        .scripture
        .list_versions(lang)
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}
