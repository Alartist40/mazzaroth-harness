use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use crate::server::ServerState;

pub async fn handle_galaxy_celestial(
    State(state): State<ServerState>,
) -> Result<Json<crate::engine::CelestialGalaxyState>, StatusCode> {
    state
        .engine
        .get_galaxy_state()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
