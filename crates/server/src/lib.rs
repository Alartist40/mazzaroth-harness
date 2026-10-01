pub mod fetch;
pub mod routes;
pub mod state;

pub use state::ServerState;

use axum::http::{header, HeaderValue};
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;

pub fn create_app(state: ServerState) -> Router {
    let dev_ui = state.dev_ui_dir.clone().unwrap_or_else(|| {
        PathBuf::from("web")
    });

    let index_file = dev_ui.join("index.html");

    // Fonts get their own service WITHOUT the SPA fallback: a missing glyph
    // range must be a clean 404 (MapLibre just skips that label) — falling
    // back to index.html makes MapLibre parse HTML as a glyph PBF and throw
    // "Unimplemented type: 4" garbage errors.
    let fonts_dir = dev_ui.join("fonts");

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/status", get(routes::status::handle_status))
        .route("/api/sections", get(routes::handle_sections))
        .route("/api/sections/constellations", get(routes::constellation::handle_constellations_section))
        .route("/api/memory/celestial", get(routes::galaxy::handle_galaxy))
        .route("/api/memory/node", post(routes::galaxy::handle_create_node))
        .route("/api/galaxy", get(routes::galaxy::handle_galaxy))
        .route("/api/search", get(routes::search::handle_search))
        .route("/api/documents", get(routes::read::handle_list_documents))
        .route("/api/categories", get(routes::read::handle_list_categories))
        .route("/api/tree", get(routes::read::handle_get_knowledge_tree))
        .route("/api/read/{doc_id}", get(routes::read::handle_get_document))
        .route("/api/sky", get(routes::sky::handle_sky_projection))
        .route("/api/ask", post(routes::ask::handle_ask))
        .route("/api/scripture/meta", get(routes::scripture::handle_scripture_meta))
        .route("/api/scripture", get(routes::scripture::handle_scripture_chapter))
        .route("/api/scripture/chapter", get(routes::scripture::handle_scripture_chapter))
        .route("/api/scripture/languages", get(routes::scripture::handle_scripture_languages))
        .route("/api/scripture/languages/detailed", get(routes::scripture::handle_scripture_languages_detailed))
        .route("/api/scripture/versions", get(routes::scripture::handle_scripture_versions))
        .route("/api/notes", get(routes::notes::handle_list_notes))
        .route("/api/notes", post(routes::notes::handle_create_note))
        .route("/api/notes/{id}", get(routes::notes::handle_get_note))
        .route("/api/notes/{id}", put(routes::notes::handle_update_note))
        .route("/api/notes/{id}", delete(routes::notes::handle_delete_note))
        .route("/api/maps", get(routes::maps::handle_list_maps))
        .route(
            "/api/maps/fetch",
            get(routes::maps::handle_fetch_status).post(routes::maps::handle_fetch_start),
        )
        .route("/api/maps/{filename}", delete(routes::maps::handle_delete_map))
        .route("/maps/{filename}", get(routes::maps::handle_serve_pmtiles))
        .nest_service("/fonts", ServeDir::new(fonts_dir))
        .fallback_service(ServeDir::new(&dev_ui).fallback(ServeFile::new(index_file)))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
