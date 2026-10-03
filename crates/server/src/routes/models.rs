use crate::state::ServerState;
use axum::extract::State;
use axum::response::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub model: String,
    pub size: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Option<Vec<OllamaModelTag>>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelTag {
    name: String,
    model: Option<String>,
    size: Option<u64>,
}

pub async fn handle_list_models(
    State(state): State<ServerState>,
) -> Json<Vec<ModelInfo>> {
    let endpoint = state.config.llm_endpoint.trim_end_matches('/');
    let url = format!("{}/api/tags", endpoint);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    if let Ok(resp) = client.get(&url).send().await {
        if resp.status().is_success() {
            if let Ok(tags) = resp.json::<OllamaTagsResponse>().await {
                if let Some(models) = tags.models {
                    let list = models
                        .into_iter()
                        .filter(|m| !m.name.contains("embed"))
                        .map(|m| ModelInfo {
                            model: m.model.unwrap_or_else(|| m.name.clone()),
                            name: m.name,
                            size: m.size,
                        })
                        .collect();
                    return Json(list);
                }
            }
        }
    }

    Json(vec![])
}
