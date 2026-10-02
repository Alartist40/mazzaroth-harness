pub mod fetch;
pub mod routes;
pub mod state;

pub use state::ServerState;

use axum::http::{header, HeaderValue};
use axum::routing::{delete, get, post};
use axum::Router;
use std::path::PathBuf;
use tower::Layer;
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

    let api_router = Router::new()
        .route("/status", get(routes::status::handle_status))
        .route("/sections", get(routes::handle_sections))
        .route("/sections/constellations", get(routes::constellation::handle_constellations_section))
        .route("/memory/celestial", get(routes::galaxy::handle_galaxy))
        .route("/memory/node", post(routes::galaxy::handle_create_node))
        .route("/galaxy", get(routes::galaxy::handle_galaxy))
        .route("/search", get(routes::search::handle_search))
        .route("/documents", get(routes::read::handle_list_documents))
        .route("/categories", get(routes::read::handle_list_categories))
        .route("/tree", get(routes::read::handle_get_knowledge_tree))
        .route("/read/{doc_id}", get(routes::read::handle_get_document))
        .route("/sky", get(routes::sky::handle_sky_projection))
        .route("/ask", post(routes::ask::handle_ask))
        .route("/scripture/meta", get(routes::scripture::handle_scripture_meta))
        .route("/scripture", get(routes::scripture::handle_scripture_chapter))
        .route("/scripture/chapter", get(routes::scripture::handle_scripture_chapter))
        .route("/scripture/languages", get(routes::scripture::handle_scripture_languages))
        .route("/scripture/languages/detailed", get(routes::scripture::handle_scripture_languages_detailed))
        .route("/scripture/versions", get(routes::scripture::handle_scripture_versions))
        .route("/notes", get(routes::notes::handle_list_notes).post(routes::notes::handle_create_note))
        .route("/notes/{id}", get(routes::notes::handle_get_note).put(routes::notes::handle_update_note).delete(routes::notes::handle_delete_note))
        .route("/maps", get(routes::maps::handle_list_maps))
        .route(
            "/maps/fetch",
            get(routes::maps::handle_fetch_status).post(routes::maps::handle_fetch_start),
        )
        .route("/maps/{filename}", delete(routes::maps::handle_delete_map))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        ));

    let fonts_router = Router::new()
        .fallback_service(ServeDir::new(fonts_dir))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=86400, stale-while-revalidate=604800"),
        ));


    // Static assets (index.html / JS / CSS) get an explicit no-cache header —
    // without it browsers use heuristic freshness and serve stale builds after
    // an update (the exact bug this header previously fixed router-wide).
    let static_service = SetResponseHeaderLayer::overriding(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    )
    .layer(ServeDir::new(&dev_ui).fallback(ServeFile::new(index_file)));

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api", api_router)
        .route("/maps/{filename}", get(routes::maps::handle_serve_pmtiles))
        .nest("/fonts", fonts_router)
        .fallback_service(static_service)
        .with_state(state)
}
