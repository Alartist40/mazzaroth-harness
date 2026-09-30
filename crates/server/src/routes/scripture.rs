use crate::state::ServerState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Json;
use librarian_core::ScriptureMetaResponse;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
pub struct ScriptureChapterResponse {
    pub language: String,
    pub version: String,
    pub book: String,
    pub chapter: usize,
    pub verses: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScriptureVersionsQuery {
    pub lang: Option<String>,
}

pub async fn handle_scripture_meta(
    State(state): State<ServerState>,
    Query(query): Query<ScriptureMetaQuery>,
) -> Result<Json<ScriptureMetaResponse>, StatusCode> {
    let lang = query.lang.as_deref().unwrap_or("eng");
    let version = query.version.as_deref().unwrap_or("kjv");

    state
        .scripture
        .get_metadata(lang, version)
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn handle_scripture_chapter(
    State(state): State<ServerState>,
    Query(query): Query<ScriptureChapterQuery>,
) -> Result<Json<ScriptureChapterResponse>, StatusCode> {
    let lang = query.lang.as_deref().unwrap_or("eng");
    let version = query.version.as_deref().unwrap_or("kjv");
    let book = query.book.as_deref().unwrap_or("Genesis");
    let chapter = query.chapter.unwrap_or(1);

    let verses = state
        .scripture
        .get_chapter(lang, version, book, chapter)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(ScriptureChapterResponse {
        language: lang.to_string(),
        version: version.to_string(),
        book: book.to_string(),
        chapter,
        verses,
    }))
}

pub async fn handle_scripture_languages(
    State(state): State<ServerState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let langs = state.scripture.list_languages();
    Ok(Json(langs))
}

pub async fn handle_scripture_versions(
    State(state): State<ServerState>,
    Query(query): Query<ScriptureVersionsQuery>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let lang = query.lang.as_deref().unwrap_or("eng");
    let versions = state.scripture.list_versions(lang);
    if versions.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(Json(versions))
}
