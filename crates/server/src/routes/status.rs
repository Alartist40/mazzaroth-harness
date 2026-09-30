use crate::state::ServerState;
use axum::extract::State;
use axum::response::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemStatus {
    pub llm_online: bool,
    pub llm_endpoint: String,
    pub llm_model: String,
    pub available_models: Vec<String>,
    pub profile: String,
    pub total_ram_mb: u64,
    pub total_documents: usize,
    pub total_chunks: usize,
    pub total_notes: usize,
    pub total_constellations: usize,
    pub total_maps: usize,
    pub total_nodes: usize,
    pub total_links: usize,
    pub version: String,
}

pub async fn handle_status(State(state): State<ServerState>) -> Json<SystemStatus> {
    let is_online = state.is_llm_reachable().await;
    let endpoint = state.config.llm_endpoint.clone();
    let model = state.config.llm_model.clone();

    // Probe Ollama /api/tags for installed models
    let mut available_models = Vec::new();
    if is_online {
        let client = reqwest::Client::new();
        let tags_url = format!("{}/api/tags", endpoint.trim_end_matches('/'));
        if let Ok(resp) = client.get(&tags_url).timeout(std::time::Duration::from_millis(1500)).send().await {
            if let Ok(body) = resp.json::<serde_json::Value>().await {
                if let Some(models) = body["models"].as_array() {
                    for m in models {
                        if let Some(name) = m["name"].as_str() {
                            available_models.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    let total_documents = state.db.count_documents().unwrap_or(0);
    let total_chunks = state.db.count_chunks().unwrap_or(0);
    let notes = state.db.list_notes().unwrap_or_default().len();
    let constellations = librarian_core::get_all_constellations().len();

    let maps_count = if let Ok(entries) = std::fs::read_dir("maps") {
        entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "pmtiles"))
            .count()
    } else {
        0
    };

    let mut total_ram_mb = 8192;
    if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
        if let Some(total_line) = meminfo.lines().find(|l| l.starts_with("MemTotal:")) {
            if let Some(kb_str) = total_line.split_whitespace().nth(1) {
                if let Ok(kb) = kb_str.parse::<u64>() {
                    total_ram_mb = kb / 1024;
                }
            }
        }
    }

    let total_nodes = state.db.list_nodes().unwrap_or_default().len() + total_documents + 1;
    let total_links = state.db.list_links().unwrap_or_default().len() + total_documents;

    Json(SystemStatus {
        llm_online: is_online,
        llm_endpoint: endpoint,
        llm_model: model,
        available_models,
        profile: state.config.profile.as_str().to_string(),
        total_ram_mb,
        total_documents,
        total_chunks,
        total_notes: notes,
        total_constellations: constellations,
        total_maps: maps_count,
        total_nodes,
        total_links,
        version: "3.2.0".to_string(),
    })
}
