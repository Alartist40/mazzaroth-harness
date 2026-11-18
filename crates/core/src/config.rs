use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum HardwareProfile {
    Tiny,
    #[default]
    Standard,
    Full,
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

    /// Auto-detects hardware profile from Linux `/proc/meminfo` (`MemTotal`).
    pub fn detect_auto() -> Self {
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if let Some(rest) = line.strip_prefix("MemTotal:") {
                    let kb: u64 = rest
                        .split_whitespace()
                        .next()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0);
                    let gb = kb as f64 / (1024.0 * 1024.0);
                    return if gb < 2.5 {
                        Self::Tiny
                    } else if gb < 6.5 {
                        Self::Standard
                    } else {
                        Self::Full
                    };
                }
            }
        }
        Self::Standard
    }
}

impl std::str::FromStr for HardwareProfile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Self::detect_auto()),
            "tiny" => Ok(Self::Tiny),
            "standard" => Ok(Self::Standard),
            "full" => Ok(Self::Full),
            other => Err(format!("Unknown hardware profile '{}' (valid: auto, tiny, standard, full)", other)),
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
            llm_model: "lfm3:1.7b".to_string(),
            bind_addr: "127.0.0.1:8080".to_string(),
        }
    }
}
