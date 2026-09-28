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
        // === 12 ZODIAC CONSTELLATIONS ===
        ("aries", "Aries (The Ram)", vec![
            ("hamal", "Hamal (α Ari)", -280.0, 160.0, 110.0, "Principal star in the head of the Ram", vec!["zodiac", "aries"]),
            ("sheratan", "Sheratan (β Ari)", -310.0, 140.0, 120.0, "Second brightest star in Aries", vec!["zodiac", "aries"]),
            ("mesarthim", "Mesarthim (γ Ari)", -330.0, 130.0, 125.0, "First discovered telescopic binary star", vec!["zodiac", "aries"]),
            ("botein", "Botein (δ Ari)", -250.0, 150.0, 100.0, "Orange giant star in the tail of Aries", vec!["zodiac", "aries"]),
        ], vec![(0, 1), (1, 2), (0, 3)]),

        ("taurus", "Taurus (The Bull)", vec![
            ("aldebaran", "Aldebaran (α Tau)", -220.0, 50.0, 120.0, "The glowing Eye of the Bull", vec!["zodiac", "taurus", "giant"]),
            ("elnath", "Elnath (β Tau)", -180.0, 130.0, 80.0, "Tip of the Northern Horn", vec!["zodiac", "taurus"]),
            ("tianguan", "Zeta Tauri", -160.0, 80.0, 100.0, "Tip of the Southern Horn", vec!["zodiac", "taurus"]),
            ("alcyone", "Alcyone (Pleiades)", -260.0, 90.0, 150.0, "Central star of the Seven Sisters", vec!["zodiac", "taurus", "pleiades"]),
        ], vec![(0, 1), (0, 2), (0, 3)]),

        ("gemini", "Gemini (The Twins)", vec![
            ("pollux", "Pollux (β Gem)", -80.0, 200.0, 190.0, "Immortal twin brother, giant star with exoplanet", vec!["zodiac", "gemini"]),
            ("castor", "Castor (α Gem)", -60.0, 230.0, 210.0, "Mortal twin brother, complex sextuple system", vec!["zodiac", "gemini"]),
            ("alhena", "Alhena (γ Gem)", -120.0, 100.0, 150.0, "Bright star at the foot of Pollux", vec!["zodiac", "gemini"]),
            ("wasat", "Wasat (δ Gem)", -80.0, 150.0, 170.0, "Middle star of the Twins", vec!["zodiac", "gemini"]),
            ("mebsuta", "Mebsuta (ε Gem)", -40.0, 180.0, 190.0, "Outstretched arm star of Castor", vec!["zodiac", "gemini"]),
        ], vec![(1, 0), (0, 3), (3, 2), (1, 4)]),

        ("cancer", "Cancer (The Crab)", vec![
            ("acubens", "Acubens (α Cnc)", 80.0, 80.0, 190.0, "The southern claw of the Crab", vec!["zodiac", "cancer"]),
            ("altarf", "Altarf (β Cnc)", 120.0, 50.0, 170.0, "Brightest star in Cancer", vec!["zodiac", "cancer"]),
            ("asellus_borealis", "Asellus Borealis", 100.0, 130.0, 210.0, "Northern Donkey Star flanking the Beehive Cluster", vec!["zodiac", "cancer"]),
            ("asellus_australis", "Asellus Australis", 110.0, 100.0, 200.0, "Southern Donkey Star flanking Praesepe", vec!["zodiac", "cancer"]),
        ], vec![(0, 3), (3, 2), (3, 1)]),

        ("leo", "Leo (The Lion)", vec![
            ("regulus", "Regulus (α Leo)", 240.0, 80.0, 150.0, "Heart of the Lion, Little King", vec!["zodiac", "leo", "quadruple"]),
            ("denebola", "Denebola (β Leo)", 320.0, 120.0, 90.0, "Tail of the Lion", vec!["zodiac", "leo"]),
            ("algieba", "Algieba (γ Leo)", 260.0, 140.0, 130.0, "Mane star, beautiful binary", vec!["zodiac", "leo"]),
            ("zosma", "Zosma (δ Leo)", 300.0, 150.0, 100.0, "Back of the Lion", vec!["zodiac", "leo"]),
        ], vec![(0, 2), (2, 3), (3, 1), (1, 0)]),

        ("virgo", "Virgo (The Maiden)", vec![
            ("spica", "Spica (α Vir)", 290.0, -110.0, 50.0, "Ear of wheat, brilliant blue binary star", vec!["zodiac", "virgo"]),
            ("zavijava", "Zavijava (β Vir)", 260.0, 30.0, 110.0, "Corner of the Maiden", vec!["zodiac", "virgo"]),
            ("porrima", "Porrima (γ Vir)", 280.0, -20.0, 80.0, "Goddess of Prophecy binary", vec!["zodiac", "virgo"]),
            ("auva", "Auva (δ Vir)", 300.0, -40.0, 60.0, "Red giant in Virgo", vec!["zodiac", "virgo"]),
            ("vindemiatrix", "Vindemiatrix (ε Vir)", 310.0, 30.0, 70.0, "The Grape Gatherer", vec!["zodiac", "virgo"]),
        ], vec![(1, 2), (2, 3), (3, 0), (2, 4)]),

        ("libra", "Libra (The Scales)", vec![
            ("zubeneschamali", "Zubeneschamali (β Lib)", 220.0, -140.0, 120.0, "Northern Claw, emerald-tinted blue star", vec!["zodiac", "libra"]),
            ("zubenelgenubi", "Zubenelgenubi (α Lib)", 180.0, -160.0, 140.0, "Southern Claw visual binary", vec!["zodiac", "libra"]),
            ("zubenhakrabi", "Zubenelakrab (γ Lib)", 230.0, -180.0, 110.0, "Claw of the Scorpion scale", vec!["zodiac", "libra"]),
            ("brachium", "Brachium (σ Lib)", 200.0, -210.0, 130.0, "Red giant base of the balance", vec!["zodiac", "libra"]),
        ], vec![(0, 1), (0, 2), (2, 3), (3, 1)]),

        ("scorpius", "Scorpius (The Scorpion)", vec![
            ("antares", "Antares (α Sco)", 80.0, -220.0, 180.0, "Rival of Mars, red supergiant heart", vec!["zodiac", "scorpius", "supergiant"]),
            ("graffias", "Graffias (β Sco)", 60.0, -170.0, 190.0, "Head claw of the Scorpion", vec!["zodiac", "scorpius"]),
            ("dschubba", "Dschubba (δ Sco)", 70.0, -190.0, 195.0, "Forehead star of the Scorpion", vec!["zodiac", "scorpius"]),
            ("shaula", "Shaula (λ Sco)", 140.0, -290.0, 150.0, "The stinger star of the Scorpion", vec!["zodiac", "scorpius"]),
            ("sargas", "Sargas (θ Sco)", 110.0, -280.0, 170.0, "Tail curve star of the Scorpion", vec!["zodiac", "scorpius"]),
        ], vec![(1, 2), (2, 0), (0, 4), (4, 3)]),

        ("sagittarius", "Sagittarius (The Archer)", vec![
            ("kaus_australis", "Kaus Australis (ε Sgr)", -30.0, -260.0, 210.0, "Southern base of the Archer Bow", vec!["zodiac", "sagittarius"]),
            ("nunki", "Nunki (σ Sgr)", -10.0, -210.0, 240.0, "Vane of the Arrow, oldest named star", vec!["zodiac", "sagittarius"]),
            ("ascella", "Ascella (ζ Sgr)", -40.0, -240.0, 220.0, "Armpit star of the Archer", vec!["zodiac", "sagittarius"]),
            ("kaus_media", "Kaus Media (δ Sgr)", -60.0, -230.0, 200.0, "Middle star of the Bow", vec!["zodiac", "sagittarius"]),
            ("kaus_borealis", "Kaus Borealis (λ Sgr)", -70.0, -200.0, 210.0, "Top peak of the Teapot", vec!["zodiac", "sagittarius"]),
        ], vec![(0, 2), (2, 1), (1, 4), (4, 3), (3, 0), (3, 2)]),

        ("capricornus", "Capricornus (The Sea Goat)", vec![
            ("deneb_algedi", "Deneb Algedi (δ Cap)", -180.0, -180.0, 160.0, "Tail of the Goat, eclipse binary", vec!["zodiac", "capricorn"]),
            ("nashira", "Nashira (γ Cap)", -190.0, -160.0, 170.0, "Bringer of Good Tidings", vec!["zodiac", "capricorn"]),
            ("dabih", "Dabih (β Cap)", -230.0, -120.0, 190.0, "The Slaughterer multiple star system", vec!["zodiac", "capricorn"]),
            ("algedi", "Algedi (α Cap)", -240.0, -100.0, 200.0, "Head of the Goat optical binary", vec!["zodiac", "capricorn"]),
        ], vec![(3, 2), (2, 1), (1, 0), (0, 3)]),

        ("aquarius", "Aquarius (The Water Bearer)", vec![
            ("sadalsuud", "Sadalsuud (β Aqr)", -280.0, -40.0, 160.0, "Luckiest of the Lucky yellow supergiant", vec!["zodiac", "aquarius"]),
            ("sadalmelik", "Sadalmelik (α Aqr)", -270.0, 0.0, 140.0, "Lucky Star of the King", vec!["zodiac", "aquarius"]),
            ("sadachbia", "Sadachbia (γ Aqr)", -290.0, 10.0, 130.0, "Lucky Star of Hidden Things", vec!["zodiac", "aquarius"]),
            ("skat", "Skat (δ Aqr)", -320.0, -90.0, 110.0, "The Shin star of the Water Bearer", vec!["zodiac", "aquarius"]),
        ], vec![(0, 1), (1, 2), (0, 3)]),

        ("pisces", "Pisces (The Fishes)", vec![
            ("alrescha", "Alrescha (α Psc)", -310.0, 60.0, 60.0, "The Knot uniting the two celestial cord ribbons", vec!["zodiac", "pisces"]),
            ("fumalsamakah", "Fumalsamakah (β Psc)", -340.0, 100.0, 40.0, "Snout of the Western Fish", vec!["zodiac", "pisces"]),
            ("torcular", "Torcular (ο Psc)", -290.0, 90.0, 80.0, "Northern chord of the Fishes", vec!["zodiac", "pisces"]),
            ("linteum", "Delta Piscium", -270.0, 110.0, 90.0, "Eastern Fish anchor", vec!["zodiac", "pisces"]),
        ], vec![(0, 1), (0, 2), (2, 3)]),

        // === 20 MAJOR CELESTIAL ASTERISMS ===
        ("orion", "Orion (The Hunter)", vec![
            ("betelgeuse", "Betelgeuse (α Ori)", -160.0, 120.0, 220.0, "Red Supergiant in Orion shoulder", vec!["asterism", "orion", "supergiant"]),
            ("rigel", "Rigel (β Ori)", -100.0, -120.0, 240.0, "Blue Supergiant in Orion foot", vec!["asterism", "orion", "supergiant"]),
            ("bellatrix", "Bellatrix (γ Ori)", -100.0, 100.0, 200.0, "Amazon Star in Orion", vec!["asterism", "orion"]),
            ("saiph", "Saiph (κ Ori)", -150.0, -110.0, 210.0, "Southern star in Orion", vec!["asterism", "orion"]),
            ("alnitak", "Alnitak (ζ Ori)", -130.0, 0.0, 220.0, "Eastern star of Orion Belt", vec!["asterism", "orion", "belt"]),
            ("alnilam", "Alnilam (ε Ori)", -120.0, 5.0, 215.0, "Center star of Orion Belt", vec!["asterism", "orion", "belt"]),
            ("mintaka", "Mintaka (δ Ori)", -110.0, 10.0, 210.0, "Western star of Orion Belt", vec!["asterism", "orion", "belt"]),
        ], vec![(0, 2), (2, 6), (6, 5), (5, 4), (4, 0), (0, 3), (1, 4), (1, 3)]),

        ("ursa_major", "Ursa Major (Big Dipper)", vec![
            ("dubhe", "Dubhe (α UMa)", 180.0, 220.0, -150.0, "Pointer star to Polaris", vec!["asterism", "ursa_major", "dipper"]),
            ("merak", "Merak (β UMa)", 160.0, 170.0, -140.0, "Pointer star to Polaris", vec!["asterism", "ursa_major", "dipper"]),
            ("phecda", "Phecda (γ UMa)", 220.0, 160.0, -160.0, "Bowl star of Big Dipper", vec!["asterism", "ursa_major", "dipper"]),
            ("megrez", "Megrez (δ UMa)", 230.0, 200.0, -170.0, "Connecting star of the handle", vec!["asterism", "ursa_major", "dipper"]),
            ("alioth", "Alioth (ε UMa)", 280.0, 210.0, -180.0, "Brightest star in Ursa Major", vec!["asterism", "ursa_major", "dipper"]),
            ("mizar", "Mizar (ζ UMa)", 320.0, 230.0, -190.0, "Famous visual double star with Alcor", vec!["asterism", "ursa_major", "dipper"]),
            ("alkaid", "Alkaid (η UMa)", 360.0, 220.0, -200.0, "Tip of the Great Bear tail", vec!["asterism", "ursa_major", "dipper"]),
        ], vec![(0, 1), (1, 2), (2, 3), (3, 0), (3, 4), (4, 5), (5, 6)]),

        ("ursa_minor", "Ursa Minor (Little Dipper)", vec![
            ("polaris", "Polaris (α UMi)", 0.0, 380.0, -100.0, "The North Star, celestial anchor of rotation", vec!["asterism", "ursa_minor", "pole"]),
            ("kochab", "Kochab (β UMi)", 40.0, 320.0, -120.0, "Guardian of the Celestial Pole", vec!["asterism", "ursa_minor"]),
            ("pherkad", "Pherkad (γ UMi)", 60.0, 330.0, -130.0, "The Dim One in the Little Bear", vec!["asterism", "ursa_minor"]),
            ("yildun", "Yildun (δ UMi)", 20.0, 360.0, -110.0, "Handle star of the Little Dipper", vec!["asterism", "ursa_minor"]),
        ], vec![(0, 3), (3, 1), (1, 2)]),

        ("cassiopeia", "Cassiopeia (The Queen)", vec![
            ("schedar", "Schedar (α Cas)", -260.0, 280.0, -100.0, "Brightest star in Cassiopeia W", vec!["asterism", "cassiopeia"]),
            ("caph", "Caph (β Cas)", -300.0, 260.0, -90.0, "Western tip of the W", vec!["asterism", "cassiopeia"]),
            ("gamma_cas", "Navi (γ Cas)", -240.0, 310.0, -120.0, "Center peak of Cassiopeia W", vec!["asterism", "cassiopeia"]),
            ("ruchbah", "Ruchbah (δ Cas)", -200.0, 290.0, -110.0, "Knee star of Cassiopeia", vec!["asterism", "cassiopeia"]),
            ("segin", "Segin (ε Cas)", -170.0, 310.0, -130.0, "Eastern tip of Cassiopeia W", vec!["asterism", "cassiopeia"]),
        ], vec![(1, 0), (0, 2), (2, 3), (3, 4)]),

        ("cygnus", "Cygnus (Northern Cross)", vec![
            ("deneb", "Deneb (α Cyg)", 50.0, 320.0, 100.0, "Supergiant star in the Summer Triangle", vec!["asterism", "cygnus", "summer_triangle"]),
            ("albireo", "Albireo (β Cyg)", -50.0, 200.0, 60.0, "Famous gold and blue binary star", vec!["asterism", "cygnus"]),
            ("sadr", "Sadr (γ Cyg)", 0.0, 270.0, 80.0, "Center of the Northern Cross", vec!["asterism", "cygnus"]),
            ("gienah", "Gienah (ε Cyg)", 60.0, 250.0, 50.0, "Eastern wing of Cygnus", vec!["asterism", "cygnus"]),
            ("fawaris", "Delta Cygni", -60.0, 280.0, 110.0, "Western wing of Cygnus", vec!["asterism", "cygnus"]),
        ], vec![(0, 2), (2, 1), (4, 2), (2, 3)]),

        ("lyra", "Lyra (The Harp)", vec![
            ("vega", "Vega (α Lyr)", -40.0, 290.0, 130.0, "The standard star of zero magnitude", vec!["asterism", "lyra", "summer_triangle"]),
            ("sheliak", "Sheliak (β Lyr)", -30.0, 260.0, 150.0, "Prototype eclipsing binary variable", vec!["asterism", "lyra"]),
            ("sulafat", "Sulafat (γ Lyr)", -10.0, 250.0, 160.0, "The Tortoise shell star", vec!["asterism", "lyra"]),
            ("delta_lyr", "Delta Lyrae", -20.0, 280.0, 140.0, "Visual double in Lyra diamond", vec!["asterism", "lyra"]),
        ], vec![(0, 3), (3, 1), (1, 2), (2, 3)]),

        ("pegasus", "Pegasus (The Great Square)", vec![
            ("markab", "Markab (α Peg)", 120.0, 180.0, -260.0, "Corner of the Great Square", vec!["asterism", "pegasus"]),
            ("scheat", "Scheat (β Peg)", 130.0, 240.0, -280.0, "Red giant corner of the Great Square", vec!["asterism", "pegasus"]),
            ("algenib", "Algenib (γ Peg)", 190.0, 170.0, -240.0, "Wing corner of the Great Square", vec!["asterism", "pegasus"]),
            ("enif", "Enif (ε Peg)", 50.0, 150.0, -280.0, "Nose star of the winged horse", vec!["asterism", "pegasus"]),
        ], vec![(0, 1), (1, 2), (2, 0), (0, 3)]),

        ("andromeda", "Andromeda (The Chained Maiden)", vec![
            ("alpheratz", "Alpheratz (α And)", 200.0, 230.0, -260.0, "Shared anchor star with Pegasus Square", vec!["asterism", "andromeda"]),
            ("mirach", "Mirach (β And)", 240.0, 270.0, -230.0, "Guide star to the Andromeda Galaxy (M31)", vec!["asterism", "andromeda"]),
            ("almach", "Almach (γ And)", 280.0, 310.0, -200.0, "Multi-color quadruple star system", vec!["asterism", "andromeda"]),
        ], vec![(0, 1), (1, 2)]),

        ("draco", "Draco (The Dragon)", vec![
            ("thuban", "Thuban (α Dra)", 120.0, 360.0, -80.0, "Ancient North Pole Star of Egypt", vec!["asterism", "draco", "pole"]),
            ("rastaban", "Rastaban (β Dra)", 80.0, 310.0, 20.0, "Eye of the Dragon", vec!["asterism", "draco"]),
            ("eltanin", "Eltanin (γ Dra)", 90.0, 290.0, 40.0, "Head of the Dragon", vec!["asterism", "draco"]),
            ("alwaid", "Nu Draconis", 110.0, 300.0, 30.0, "Nose of the Dragon", vec!["asterism", "draco"]),
        ], vec![(0, 1), (1, 2), (2, 3), (3, 1)]),

        ("crux", "Crux (Southern Cross)", vec![
            ("acrux", "Acrux (α Cru)", -80.0, -320.0, -120.0, "Brightest star in the Southern Cross", vec!["asterism", "crux"]),
            ("mimosa", "Mimosa (β Cru)", -50.0, -300.0, -100.0, "Eastern jewel of the Southern Cross", vec!["asterism", "crux"]),
            ("gacrux", "Gacrux (γ Cru)", -80.0, -270.0, -130.0, "Red giant head of the Cross", vec!["asterism", "crux"]),
            ("imodi", "Delta Crucis", -100.0, -290.0, -110.0, "Western arm of the Cross", vec!["asterism", "crux"]),
        ], vec![(0, 2), (3, 1)]),

        ("centaurus", "Centaurus (The Centaur)", vec![
            ("rigil_kent", "Alpha Centauri", -120.0, -340.0, -70.0, "Nearest star system to our Sun", vec!["asterism", "centaurus"]),
            ("hadar", "Hadar (β Cen)", -90.0, -330.0, -80.0, "Southern Pointer to Crux", vec!["asterism", "centaurus"]),
            ("menkent", "Menkent (θ Cen)", -140.0, -260.0, -40.0, "Shoulder of the Centaur", vec!["asterism", "centaurus"]),
        ], vec![(0, 1), (1, 2)]),

        ("hercules", "Hercules (The Kneeler)", vec![
            ("rasalgethi", "Rasalgethi (α Her)", 10.0, 140.0, 140.0, "Head of the Kneeler, red supergiant binary", vec!["asterism", "hercules"]),
            ("kornephoros", "Kornephoros (β Her)", 30.0, 160.0, 160.0, "Club bearer star of Hercules", vec!["asterism", "hercules"]),
            ("sarir", "Zeta Herculis", 50.0, 190.0, 130.0, "Keystone star of the Hercules Quadrangle", vec!["asterism", "hercules"]),
            ("pi_her", "Pi Herculis", 20.0, 210.0, 110.0, "Top corner of the Keystone", vec!["asterism", "hercules"]),
        ], vec![(0, 1), (1, 2), (2, 3), (3, 0)]),

        ("cepheus", "Cepheus (The King)", vec![
            ("alderamin", "Alderamin (α Cep)", -180.0, 360.0, -160.0, "Future North Star in 7500 AD", vec!["asterism", "cepheus"]),
            ("alfirk", "Alfirk (β Cep)", -220.0, 370.0, -140.0, "Prototype pulsating star", vec!["asterism", "cepheus"]),
            ("errai", "Errai (γ Cep)", -150.0, 390.0, -180.0, "First exoplanet discovered around giant star", vec!["asterism", "cepheus"]),
        ], vec![(0, 1), (0, 2)]),

        ("perseus", "Perseus (The Hero)", vec![
            ("mirfak", "Mirfak (α Per)", -160.0, 240.0, 10.0, "Brightest star in Perseus constellation", vec!["asterism", "perseus"]),
            ("algol", "Algol (β Per)", -190.0, 200.0, 30.0, "The Demon Star, famous eclipsing variable", vec!["asterism", "perseus", "variable"]),
            ("menkib", "Menkib (ξ Per)", -130.0, 210.0, 0.0, "Ionizing star of California Nebula", vec!["asterism", "perseus"]),
        ], vec![(0, 1), (0, 2)]),

        ("bootes", "Bootes (The Herdsman)", vec![
            ("arcturus", "Arcturus (α Boo)", 180.0, 190.0, 20.0, "Brightest star in northern celestial hemisphere", vec!["asterism", "bootes", "giant"]),
            ("nekkar", "Nekkar (β Boo)", 200.0, 240.0, -10.0, "Head of the Herdsman", vec!["asterism", "bootes"]),
            ("izhar", "Izar (ε Boo)", 190.0, 220.0, 10.0, "Pulcherrima, most beautiful double star", vec!["asterism", "bootes"]),
            ("muphrid", "Muphrid (η Boo)", 160.0, 170.0, 40.0, "Foot star of the Herdsman", vec!["asterism", "bootes"]),
        ], vec![(0, 3), (0, 2), (2, 1)]),

        ("vela", "Vela (The Sails)", vec![
            ("suhail", "Suhail (γ Vel)", -120.0, -280.0, -180.0, "Regor, Spectral Gem of Southern Skies", vec!["asterism", "vela"]),
            ("al_suhail", "Al Suhail (λ Vel)", -150.0, -250.0, -160.0, "Supergiant orange masthead", vec!["asterism", "vela"]),
            ("markeb", "Markeb (κ Vel)", -90.0, -300.0, -190.0, "Anchor of the Sails", vec!["asterism", "vela"]),
        ], vec![(0, 1), (0, 2)]),

        ("puppis", "Puppis (The Stern)", vec![
            ("naos", "Naos (ζ Pup)", -160.0, -220.0, -220.0, "Hottest O-type star visible to naked eye", vec!["asterism", "puppis"]),
            ("ahadi", "Pi Puppis", -190.0, -200.0, -200.0, "Orange supergiant in the Stern", vec!["asterism", "puppis"]),
            ("tseen_ke", "Xi Puppis", -140.0, -230.0, -240.0, "Keel deck star", vec!["asterism", "puppis"]),
        ], vec![(0, 1), (0, 2)]),

        ("pavo", "Pavo (The Peacock)", vec![
            ("peacock", "Peacock (α Pav)", -220.0, -320.0, 40.0, "Spectroscopic binary eye of the Peacock", vec!["asterism", "pavo"]),
            ("beta_pav", "Beta Pavonis", -250.0, -300.0, 60.0, "Neck star of the Peacock", vec!["asterism", "pavo"]),
            ("delta_pav", "Delta Pavonis", -200.0, -340.0, 30.0, "Solar twin star close to Earth", vec!["asterism", "pavo"]),
        ], vec![(0, 1), (0, 2)]),

        ("auriga", "Auriga (The Charioteer)", vec![
            ("capella", "Capella (α Aur)", -120.0, 260.0, -80.0, "The Little Goat, brilliant quadruple star", vec!["asterism", "auriga"]),
            ("menkalinan", "Menkalinan (β Aur)", -90.0, 240.0, -90.0, "Shoulder of the Charioteer", vec!["asterism", "auriga"]),
            ("mahasim", "Theta Aurigae", -70.0, 210.0, -70.0, "Ankle of the Charioteer", vec!["asterism", "auriga"]),
            ("al_anj", "Iota Aurigae", -130.0, 220.0, -50.0, "Horn star of Auriga", vec!["asterism", "auriga"]),
        ], vec![(0, 1), (1, 2), (2, 3), (3, 0)]),

        ("lupus", "Lupus (The Wolf)", vec![
            ("men", "Men (α Lup)", 140.0, -240.0, 90.0, "The Blue Giant Wolf star", vec!["asterism", "lupus"]),
            ("kelewan", "Beta Lupis", 160.0, -220.0, 100.0, "Northern paw of the Wolf", vec!["asterism", "lupus"]),
            ("gamma_lup", "Gamma Lupis", 130.0, -260.0, 80.0, "Body star of the Wolf", vec!["asterism", "lupus"]),
        ], vec![(0, 1), (0, 2)]),
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

