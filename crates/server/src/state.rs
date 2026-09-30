use librarian_core::{LibrarianConfig, LibrarianDb, ScriptureReader};
use std::path::PathBuf;

#[derive(Clone)]
pub struct ServerState {
    pub db: LibrarianDb,
    pub scripture: ScriptureReader,
    pub config: LibrarianConfig,
    pub dev_ui_dir: Option<PathBuf>,
}

impl ServerState {
    pub fn new(db: LibrarianDb, config: LibrarianConfig, dev_ui_dir: Option<PathBuf>) -> Self {
        let bibles_dir = PathBuf::from("galaxy/data/bibles");
        let scripture = ScriptureReader::new(if bibles_dir.exists() {
            bibles_dir
        } else {
            PathBuf::from("bibles")
        });

        Self {
            db,
            scripture,
            config,
            dev_ui_dir,
        }
    }

    /// Check if local LLM server (Ollama or OpenAI-compatible) is reachable
    pub async fn is_llm_reachable(&self) -> bool {
        if !self.config.profile.supports_llm() {
            return false;
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(800))
            .build();

        if let Ok(c) = client {
            let url = format!("{}/api/tags", self.config.llm_endpoint.trim_end_matches('/'));
            if let Ok(resp) = c.get(&url).send().await {
                if resp.status().is_success() {
                    return true;
                }
            }

            let openai_url = format!("{}/v1/models", self.config.llm_endpoint.trim_end_matches('/'));
            if let Ok(resp) = c.get(&openai_url).send().await {
                if resp.status().is_success() {
                    return true;
                }
            }
        }

        false
    }
}
