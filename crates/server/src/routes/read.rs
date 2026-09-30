use crate::state::ServerState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use librarian_core::{ContentDocument, DocumentSummary};

pub async fn handle_list_documents(
    State(state): State<ServerState>,
) -> Result<Json<Vec<DocumentSummary>>, StatusCode> {
    state
        .db
        .list_documents()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn handle_list_categories(
    State(state): State<ServerState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    state
        .db
        .list_categories()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn handle_get_document(
    State(state): State<ServerState>,
    Path(doc_id): Path<String>,
) -> Result<Json<ContentDocument>, StatusCode> {
    match state.db.get_document(&doc_id) {
        Ok(Some(doc)) => Ok(Json(doc)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
