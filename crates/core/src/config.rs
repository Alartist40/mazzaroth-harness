use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HardwareProfile {
    Tiny,
    Standard,
    Full,
}

impl Default for HardwareProfile {
    fn default() -> Self {
        Self::Standard
    }
}

impl HardwareProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Standard => "standard",
            Self::Full => "full",
        }
    }

    pub fn max_context_tokens(&self) -> usize {
        match self {
            Self::Tiny => 0,
            Self::Standard => 2048,
            Self::Full => 4096,
        }
    }

    pub fn top_k(&self) -> usize {
        match self {
            Self::Tiny => 5,
            Self::Standard => 6,
            Self::Full => 8,
        }
    }

    pub fn supports_llm(&self) -> bool {
        match self {
            Self::Tiny => false,
            Self::Standard | Self::Full => true,
        }
    }
}

impl std::str::FromStr for HardwareProfile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tiny" => Ok(Self::Tiny),
            "standard" => Ok(Self::Standard),
            "full" => Ok(Self::Full),
            other => Err(format!("Unknown hardware profile '{}' (valid: tiny, standard, full)", other)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarianConfig {
    pub profile: HardwareProfile,
    pub db_path: PathBuf,
    pub content_dir: PathBuf,
    pub maps_dir: PathBuf,
    pub bibles_dir: PathBuf,
    pub constellations_dir: PathBuf,
    pub llm_endpoint: String,
    pub llm_model: String,
    pub bind_addr: String,
}

impl Default for LibrarianConfig {
    fn default() -> Self {
        Self {
            profile: HardwareProfile::Standard,
            db_path: PathBuf::from("data/mazzaroth.db"),
            content_dir: PathBuf::from("content"),
            maps_dir: PathBuf::from("maps"),
            bibles_dir: PathBuf::from("galaxy/data/bibles"),
            constellations_dir: PathBuf::from("constellation/data"),
            llm_endpoint: "http://127.0.0.1:11434".to_string(),
            llm_model: "ministral-3:3b".to_string(),
            bind_addr: "127.0.0.1:8080".to_string(),
        }
    }
}
