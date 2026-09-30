pub mod ask;
pub mod constellation;
pub mod galaxy;
pub mod maps;
pub mod notes;
pub mod read;
pub mod scripture;
pub mod search;
pub mod sky;
pub mod status;

use axum::response::Json;
use serde_json::json;

pub async fn handle_sections() -> Json<serde_json::Value> {
    Json(json!([
        {
            "id": "galaxy",
            "label": "Celestial Galaxy",
            "api": "/api/galaxy",
            "description": "Multilingual scripture, survival manuals, medical protocols & knowledge spiral"
        },
        {
            "id": "constellations",
            "label": "Classical Constellations",
            "api": "/api/sections/constellations",
            "description": "32 classical asterisms and complete 12 Zodiac sign geometries"
        },
        {
            "id": "sky",
            "label": "Sky Deck",
            "api": "/api/sky",
            "description": "Live local night sky dome renderer with date/time and GPS lat/lon positioning"
        },
        {
            "id": "librarian",
            "label": "AI Librarian",
            "api": "/api/ask",
            "description": "Grounded local AI librarian answering questions with exact citations"
        },
        {
            "id": "maps",
            "label": "Offline Maps",
            "api": "/api/maps",
            "description": "Offline PMTiles regional vector maps"
        }
    ]))
}

