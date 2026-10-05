use crate::state::ServerState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SearchHitResponse {
    pub chunk_id: i64,
    pub doc_id: String,
    pub doc_title: String,
    pub category: String,
    pub license: String,
    pub retrieved_date: String,
    pub section_id: String,
    pub title_path: String,
    pub text: String,
    pub snippet: String,
    pub rank: f64,
}

pub async fn handle_search(
    State(state): State<ServerState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<SearchHitResponse>>, StatusCode> {
    let limit = query
        .limit
        .unwrap_or_else(|| state.config.profile.top_k())
        .clamp(1, 100);
    match state.db.search(&query.q, limit) {
        Ok(hits) => {
            let resp = hits
                .into_iter()
                .map(|h| SearchHitResponse {
                    chunk_id: h.chunk_id,
                    doc_id: h.doc_id,
                    doc_title: h.doc_title,
                    category: h.category,
                    license: h.license,
                    retrieved_date: h.retrieved_date,
                    section_id: h.section_id,
                    title_path: h.title_path,
                    text: h.text,
                    snippet: h.snippet,
                    rank: h.rank,
                })
                .collect();
            Ok(Json(resp))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
