use crate::cognitive::node::AssociativeLink;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationLine {
    pub source_id: String,
    pub target_id: String,
    pub weight: f32,
    pub relationship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationCluster {
    pub name: String,
    pub center_x: f32,
    pub center_y: f32,
    pub center_z: f32,
    pub node_ids: Vec<String>,
}

pub fn build_constellation_lines(links: &[AssociativeLink]) -> Vec<ConstellationLine> {
    links
        .iter()
        .map(|l| ConstellationLine {
            source_id: l.source_id.clone(),
            target_id: l.target_id.clone(),
            weight: l.weight,
            relationship: l.relationship.clone(),
        })
        .collect()
}
