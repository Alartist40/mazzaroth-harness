use crate::state::ServerState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use librarian_core::Note;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub title: String,
    pub content: String,
}

pub async fn handle_list_notes(
    State(state): State<ServerState>,
) -> Result<Json<Vec<Note>>, StatusCode> {
    state
        .db
        .list_notes()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn handle_get_note(
    State(state): State<ServerState>,
    Path(id): Path<String>,
) -> Result<Json<Note>, StatusCode> {
    match state.db.get_note(&id) {
        Ok(Some(note)) => Ok(Json(note)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn handle_create_note(
    State(state): State<ServerState>,
    Json(req): Json<CreateNoteRequest>,
) -> Result<Json<Note>, StatusCode> {
    state
        .db
        .create_note(&req.title, &req.content)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn handle_update_note(
    State(state): State<ServerState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateNoteRequest>,
) -> Result<Json<Note>, StatusCode> {
    match state.db.update_note(&id, &req.title, &req.content) {
        Ok(Some(note)) => Ok(Json(note)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn handle_delete_note(
    State(state): State<ServerState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.db.delete_note(&id) {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
