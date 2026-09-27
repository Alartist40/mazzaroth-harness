use crate::cognitive::MemoryTier;
use crate::engine::MazzarothEngine;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Json};
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use serde_json::json;
use tower_http::cors::CorsLayer;

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

#[derive(Clone)]
pub struct ServerState {
    pub engine: MazzarothEngine,
}

pub fn create_router(state: ServerState) -> Router {
    Router::new()
        .route("/", get(handle_web_galaxy))
        .route("/health", get(|| async { "OK" }))
        .route("/api/memory/ingest", post(handle_ingest))
        .route("/api/memory/recall", get(handle_recall))
        .route("/api/memory/node", get(handle_get_single_node))
        .route("/api/memory/celestial", get(handle_celestial))
        .route("/api/memory/nodes", get(handle_all_nodes))
        .route("/api/mcp", post(crate::server::mcp::handle_mcp_post))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn handle_web_galaxy() -> impl IntoResponse {
    Html(include_str!("../../web/index.html"))
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
