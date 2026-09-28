use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationStar {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub desc: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationData {
    pub id: String,
    pub label: String,
    pub stars: Vec<ConstellationStar>,
    pub links: Vec<(usize, usize)>,
}

pub fn get_all_constellations() -> Vec<ConstellationData> {
    serde_json::from_str(include_str!("../data/constellations.json"))
        .expect("Failed to parse constellation catalog JSON")
}
