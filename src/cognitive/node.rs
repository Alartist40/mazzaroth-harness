use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryTier {
    Working,   // Tier 1: Real-time sensory / turn buffer
    Episodic,  // Tier 2: Chronological event timeline
    Semantic,  // Tier 3: Entity knowledge graph & concepts
    Celestial, // Tier 4: Immutable core identity & axioms
}

impl MemoryTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Working => "working",
            Self::Episodic => "episodic",
            Self::Semantic => "semantic",
            Self::Celestial => "celestial",
        }
    }
}

impl FromStr for MemoryTier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "working" => Ok(Self::Working),
            "episodic" => Ok(Self::Episodic),
            "semantic" => Ok(Self::Semantic),
            "celestial" => Ok(Self::Celestial),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: String,
    pub tier: MemoryTier,
    pub label: String,
    pub content: String,
    pub tags: Vec<String>,
    pub strength: f32,       // Mass / retention (0.0 to 1.0, Celestial = 1.0)
    pub activation: f32,     // Luminosity / current excitation (0.0 to 1.0)
    pub access_count: u64,
    pub created_at: i64,     // Unix timestamp in seconds
    pub last_accessed: i64,  // Unix timestamp in seconds
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
}

impl MemoryNode {
    pub fn new(id: impl Into<String>, tier: MemoryTier, label: impl Into<String>, content: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let initial_strength = match tier {
            MemoryTier::Celestial => 1.0,
            MemoryTier::Semantic => 0.8,
            MemoryTier::Episodic => 0.6,
            MemoryTier::Working => 0.4,
        };

        Self {
            id: id.into(),
            tier,
            label: label.into(),
            content: content.into(),
            tags: Vec::new(),
            strength: initial_strength,
            activation: 1.0,
            access_count: 1,
            created_at: now,
            last_accessed: now,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            vel_x: 0.0,
            vel_y: 0.0,
            vel_z: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociativeLink {
    pub source_id: String,
    pub target_id: String,
    pub weight: f32,          // Hebbian synaptic / gravitational strength (0.0 to 1.0)
    pub relationship: String,
    pub created_at: i64,
    pub last_reinforced: i64,
}
