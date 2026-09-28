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
        .route("/api/sections", get(handle_sections))
        .route("/api/sections/constellations", get(handle_constellations_section))
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

async fn handle_sections() -> Json<serde_json::Value> {
    Json(json!([
        {
            "id": "galaxy",
            "label": "🌌 Celestial Galaxy",
            "api": "/api/memory/celestial",
            "description": "Multilingual scriptural superclusters & neural memory spiral"
        },
        {
            "id": "constellations",
            "label": "✨ Classical Constellations",
            "api": "/api/sections/constellations",
            "description": "Astronomical asterisms and anchor stellar geometries"
        }
    ]))
}

async fn handle_constellations_section() -> Json<serde_json::Value> {
    let constellations = vec![
        ("orion", "Orion (The Hunter)", vec![
            ("betelgeuse", "Betelgeuse (α Ori)", -160.0, 120.0, 220.0, "Red Supergiant in Orion shoulder", vec!["orion", "supergiant"]),
            ("rigel", "Rigel (β Ori)", -100.0, -120.0, 240.0, "Blue Supergiant in Orion foot", vec!["orion", "supergiant"]),
            ("bellatrix", "Bellatrix (γ Ori)", -100.0, 100.0, 200.0, "Amazon Star in Orion", vec!["orion"]),
            ("saiph", "Saiph (κ Ori)", -150.0, -110.0, 210.0, "Southern star in Orion", vec!["orion"]),
            ("alnitak", "Alnitak (ζ Ori)", -130.0, 0.0, 220.0, "Eastern star of Orion Belt", vec!["orion", "belt"]),
            ("alnilam", "Alnilam (ε Ori)", -120.0, 5.0, 215.0, "Center star of Orion Belt", vec!["orion", "belt"]),
            ("mintaka", "Mintaka (δ Ori)", -110.0, 10.0, 210.0, "Western star of Orion Belt", vec!["orion", "belt"]),
        ], vec![(0, 2), (2, 6), (6, 5), (5, 4), (4, 0), (0, 3), (1, 4), (1, 3)]),
        ("ursa_major", "Ursa Major (Big Dipper)", vec![
            ("dubhe", "Dubhe (α UMa)", 180.0, 220.0, -150.0, "Pointer star to Polaris", vec!["ursa_major", "dipper"]),
            ("merak", "Merak (β UMa)", 160.0, 170.0, -140.0, "Pointer star to Polaris", vec!["ursa_major", "dipper"]),
            ("phecda", "Phecda (γ UMa)", 220.0, 160.0, -160.0, "Bowl star of Big Dipper", vec!["ursa_major", "dipper"]),
            ("megrez", "Megrez (δ UMa)", 230.0, 200.0, -170.0, "Connecting star of the handle", vec!["ursa_major", "dipper"]),
            ("alioth", "Alioth (ε UMa)", 280.0, 210.0, -180.0, "Brightest star in Ursa Major", vec!["ursa_major", "dipper"]),
            ("mizar", "Mizar (ζ UMa)", 320.0, 230.0, -190.0, "Famous visual double star with Alcor", vec!["ursa_major", "dipper"]),
            ("alkaid", "Alkaid (η UMa)", 360.0, 220.0, -200.0, "Tip of the Great Bear tail", vec!["ursa_major", "dipper"]),
        ], vec![(0, 1), (1, 2), (2, 3), (3, 0), (3, 4), (4, 5), (5, 6)]),
        ("cassiopeia", "Cassiopeia (The Queen)", vec![
            ("schedar", "Schedar (α Cas)", -260.0, 280.0, -100.0, "Brightest star in Cassiopeia W", vec!["cassiopeia"]),
            ("caph", "Caph (β Cas)", -300.0, 260.0, -90.0, "Western tip of the W", vec!["cassiopeia"]),
            ("gamma_cas", "Navi (γ Cas)", -240.0, 310.0, -120.0, "Center peak of Cassiopeia W", vec!["cassiopeia"]),
            ("ruchbah", "Ruchbah (δ Cas)", -200.0, 290.0, -110.0, "Knee star of Cassiopeia", vec!["cassiopeia"]),
            ("segin", "Segin (ε Cas)", -170.0, 310.0, -130.0, "Eastern tip of Cassiopeia W", vec!["cassiopeia"]),
        ], vec![(1, 0), (0, 2), (2, 3), (3, 4)]),
        ("cygnus", "Cygnus (Northern Cross)", vec![
            ("deneb", "Deneb (α Cyg)", 50.0, 320.0, 100.0, "Supergiant star in the Summer Triangle", vec!["cygnus", "summer_triangle"]),
            ("albireo", "Albireo (β Cyg)", -50.0, 200.0, 60.0, "Famous gold and blue binary star", vec!["cygnus"]),
            ("sadr", "Sadr (γ Cyg)", 0.0, 270.0, 80.0, "Center of the Northern Cross", vec!["cygnus"]),
            ("gienah", "Gienah (ε Cyg)", 60.0, 250.0, 50.0, "Eastern wing of Cygnus", vec!["cygnus"]),
            ("fawaris", "Delta Cygni", -60.0, 280.0, 110.0, "Western wing of Cygnus", vec!["cygnus"]),
        ], vec![(0, 2), (2, 1), (4, 2), (2, 3)]),
        ("taurus", "Taurus (The Bull)", vec![
            ("aldebaran", "Aldebaran (α Tau)", -220.0, 50.0, 120.0, "The glowing Eye of the Bull", vec!["taurus", "giant"]),
            ("elnath", "Elnath (β Tau)", -180.0, 130.0, 80.0, "Tip of the Northern Horn", vec!["taurus"]),
            ("tianguan", "Zeta Tauri", -160.0, 80.0, 100.0, "Tip of the Southern Horn", vec!["taurus"]),
            ("alcyone", "Alcyone (Pleiades)", -260.0, 90.0, 150.0, "Central star of the Seven Sisters", vec!["taurus", "pleiades"]),
        ], vec![(0, 1), (0, 2), (0, 3)]),
        ("scorpius", "Scorpius (The Scorpion)", vec![
            ("antares", "Antares (α Sco)", 80.0, -220.0, 180.0, "Rival of Mars, red supergiant heart", vec!["scorpius", "supergiant"]),
            ("graffias", "Graffias (β Sco)", 60.0, -170.0, 190.0, "Head claw of the Scorpion", vec!["scorpius"]),
            ("dschubba", "Dschubba (δ Sco)", 70.0, -190.0, 195.0, "Forehead star of the Scorpion", vec!["scorpius"]),
            ("shaula", "Shaula (λ Sco)", 140.0, -290.0, 150.0, "The stinger star of the Scorpion", vec!["scorpius"]),
        ], vec![(1, 2), (2, 0), (0, 3)]),
        ("leo", "Leo (The Lion)", vec![
            ("regulus", "Regulus (α Leo)", 240.0, 80.0, 150.0, "Heart of the Lion, Little King", vec!["leo", "quadruple"]),
            ("denebola", "Denebola (β Leo)", 320.0, 120.0, 90.0, "Tail of the Lion", vec!["leo"]),
            ("algieba", "Algieba (γ Leo)", 260.0, 140.0, 130.0, "Mane star, beautiful binary", vec!["leo"]),
            ("zobi", "Zosma (δ Leo)", 300.0, 150.0, 100.0, "Back of the Lion", vec!["leo"]),
        ], vec![(0, 2), (2, 3), (3, 1), (1, 0)]),
        ("crux", "Crux (Southern Cross)", vec![
            ("acrux", "Acrux (α Cru)", -80.0, -320.0, -120.0, "Brightest star in the Southern Cross", vec!["crux"]),
            ("mimosa", "Mimosa (β Cru)", -50.0, -300.0, -100.0, "Eastern jewel of the Southern Cross", vec!["crux"]),
            ("gacrux", "Gacrux (γ Cru)", -80.0, -270.0, -130.0, "Red giant head of the Cross", vec!["crux"]),
            ("imodi", "Delta Crucis", -100.0, -290.0, -110.0, "Western arm of the Cross", vec!["crux"]),
        ], vec![(0, 2), (3, 1)]),
        ("pegasus", "Pegasus (The Winged Horse)", vec![
            ("markab", "Markab (α Peg)", 120.0, 180.0, -260.0, "Corner of the Great Square", vec!["pegasus"]),
            ("scheat", "Scheat (β Peg)", 130.0, 240.0, -280.0, "Red giant corner of the Great Square", vec!["pegasus"]),
            ("algenib", "Algenib (γ Peg)", 190.0, 170.0, -240.0, "Wing corner of the Great Square", vec!["pegasus"]),
            ("enif", "Enif (ε Peg)", 50.0, 150.0, -280.0, "Nose star of the winged horse", vec!["pegasus"]),
        ], vec![(0, 1), (1, 2), (2, 0), (0, 3)]),
        ("andromeda", "Andromeda (The Chained Maiden)", vec![
            ("alpheratz", "Alpheratz (α And)", 200.0, 230.0, -260.0, "Shared anchor star with Pegasus Square", vec!["andromeda"]),
            ("mirach", "Mirach (β And)", 240.0, 270.0, -230.0, "Guide star to the Andromeda Galaxy (M31)", vec!["andromeda"]),
            ("almach", "Almach (γ And)", 280.0, 310.0, -200.0, "Multi-color quadruple star system", vec!["andromeda"]),
        ], vec![(0, 1), (1, 2)]),
    ];

    let mut bodies = Vec::new();
    let mut lines = Vec::new();

    for (c_id, c_label, stars, star_links) in constellations {
        let cluster_id = format!("constellation:{}:cluster", c_id);
        
        for (s_id, s_name, x, y, z, desc, tags) in &stars {
            let full_id = format!("constellation:{}:{}", c_id, s_id);
            let (px, py, pz): (f32, f32, f32) = (*x, *y, *z);
            let orbit_radius = (px * px + pz * pz).sqrt();
            bodies.push(json!({
                "id": full_id,
                "label": s_name,
                "tier": "celestial",
                "x": px, "y": py, "z": pz,
                "radius": 10.0,
                "color": { "r": 180, "g": 220, "b": 255 },
                "luminosity": 0.95,
                "mass": 0.9,
                "orbit_radius": orbit_radius,
                "orbit_angle": 0.0,
                "content": format!("{} ({})\n{}", s_name, c_label, desc),
                "tags": tags,
            }));
        }

        // Add cluster virtual center for sidebar grouping
        let (cx, cy, cz): (f32, f32, f32) = (stars[0].2, stars[0].3, stars[0].4);
        let cluster_orbit_radius = (cx * cx + cz * cz).sqrt();
        bodies.push(json!({
            "id": cluster_id,
            "label": format!("🪐 {}", c_label),
            "tier": "celestial",
            "x": cx, "y": cy, "z": cz,
            "radius": 14.0,
            "color": { "r": 255, "g": 215, "b": 0 },
            "luminosity": 1.0,
            "mass": 1.0,
            "orbit_radius": cluster_orbit_radius,
            "orbit_angle": 0.0,
            "content": format!("Constellation Anchor: {}", c_label),
            "tags": vec!["constellation", c_id],
        }));

        for (from_idx, to_idx) in star_links {
            lines.push(json!({
                "source_id": format!("constellation:{}:{}", c_id, stars[from_idx].0),
                "target_id": format!("constellation:{}:{}", c_id, stars[to_idx].0),
                "weight": 0.85,
                "relationship": "version_orbit"
            }));
        }
    }

    Json(json!({
        "bodies": bodies,
        "lines": lines,
        "timestamp": 1727500000
    }))
}

