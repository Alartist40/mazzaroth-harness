use crate::state::ServerState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyNode {
    pub id: String,
    pub label: String,
    pub category: String,
    pub kind: String, // "category", "document", "section", "note", "celestial", "semantic", "episodic"
    pub tier: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub color: String,
    pub description: String,
    pub content: String,
    pub doc_id: Option<String>,
    pub section_id: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyLink {
    pub source_id: String,
    pub target_id: String,
    pub relationship: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyGraphResponse {
    pub nodes: Vec<GalaxyNode>,
    pub bodies: Vec<GalaxyNode>,
    pub links: Vec<GalaxyLink>,
    pub lines: Vec<GalaxyLink>,
    pub total_nodes: usize,
    pub timestamp: i64,
}

pub async fn handle_galaxy(
    State(state): State<ServerState>,
) -> Result<Json<GalaxyGraphResponse>, StatusCode> {
    let memory_nodes = state.db.list_nodes().unwrap_or_default();
    let memory_links = state.db.list_links().unwrap_or_default();
    let docs = state.db.list_documents().unwrap_or_default();
    let notes = state.db.list_notes().unwrap_or_default();

    let mut nodes = Vec::new();
    let mut links = Vec::new();
    let mut existing_node_ids = std::collections::HashSet::new();

    // 1. Central library core anchor
    nodes.push(GalaxyNode {
        id: "core:librarian".to_string(),
        label: "Librarian Core".to_string(),
        category: "core".to_string(),
        kind: "core".to_string(),
        tier: "celestial".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
        radius: 16.0,
        color: "#ffffff".to_string(),
        description: "Offline Sovereign Knowledge Core".to_string(),
        content: "Offline Sovereign Knowledge Core".to_string(),
        doc_id: None,
        section_id: None,
        tags: vec!["core".to_string(), "librarian".to_string()],
    });
    existing_node_ids.insert("core:librarian".to_string());

    // 2. Add 1,097 Memory Nodes from database
    for mn in memory_nodes {
        if !existing_node_ids.contains(&mn.id) {
            let color = match mn.tier.as_str() {
                "celestial" => "#ffffff",
                "semantic" => "#f4f5f7",
                "episodic" => "#d0d5dc",
                _ => "#9aa1af",
            };

            nodes.push(GalaxyNode {
                id: mn.id.clone(),
                label: mn.label.clone(),
                category: mn.tags.first().cloned().unwrap_or_else(|| "memory".to_string()),
                kind: mn.tier.clone(),
                tier: mn.tier.clone(),
                x: if mn.pos_x.is_finite() { mn.pos_x } else { 0.0 },
                y: if mn.pos_y.is_finite() { mn.pos_y } else { 0.0 },
                z: if mn.pos_z.is_finite() { mn.pos_z } else { 0.0 },
                radius: 8.0,
                color: color.to_string(),
                description: mn.content.clone(),
                content: mn.content.clone(),
                doc_id: None,
                section_id: None,
                tags: mn.tags,
            });
            existing_node_ids.insert(mn.id);
        }
    }

    // 3. Add Memory Links from database
    for ml in memory_links {
        links.push(GalaxyLink {
            source_id: ml.source_id,
            target_id: ml.target_id,
            relationship: ml.relationship,
            weight: ml.weight,
        });
    }

    // 4. Ingested Categories & Documents
    let categories = state.db.list_categories().unwrap_or_default();
    let num_cats = categories.len().max(1);
    let mut cat_angle_map = HashMap::new();

    for (cat_idx, cat) in categories.iter().enumerate() {
        let cat_id = format!("category:{}", cat);
        let angle = (cat_idx as f32 / num_cats as f32) * std::f32::consts::PI * 2.0;
        cat_angle_map.insert(cat.clone(), angle);

        let cat_r = 140.0;
        let cx = angle.cos() * cat_r;
        let cz = angle.sin() * cat_r;

        if !existing_node_ids.contains(&cat_id) {
            nodes.push(GalaxyNode {
                id: cat_id.clone(),
                label: cat.to_uppercase(),
                category: cat.clone(),
                kind: "category".to_string(),
                tier: "category".to_string(),
                x: cx,
                y: 0.0,
                z: cz,
                radius: 12.0,
                color: "#ffffff".to_string(),
                description: format!("Knowledge Category: {}", cat),
                content: format!("Knowledge Category: {}", cat),
                doc_id: None,
                section_id: None,
                tags: vec!["category".to_string(), cat.clone()],
            });
            existing_node_ids.insert(cat_id.clone());

            links.push(GalaxyLink {
                source_id: "core:librarian".to_string(),
                target_id: cat_id,
                relationship: "contains_category".to_string(),
                weight: 1.0,
            });
        }
    }

    // Document nodes along bounded logarithmic spiral
    for (doc_idx, doc_summary) in docs.iter().enumerate() {
        let cat_angle = cat_angle_map.get(&doc_summary.category).copied().unwrap_or(0.0);
        let r = (220.0 + (doc_idx as f32 * 45.0)).min(1100.0);
        let twist = r * 0.003;
        let spiral_angle = cat_angle + twist;

        let dx = spiral_angle.cos() * r;
        let dy = (r * 0.015).sin() * 20.0;
        let dz = spiral_angle.sin() * r;

        let doc_node_id = format!("doc:{}", doc_summary.id);

        let color = match doc_summary.category.as_str() {
            "survival" => "#f4f5f7",
            "medical" | "health" => "#e4e8ec",
            "scripture" | "bible" => "#ffffff",
            "literature" | "gutenberg" => "#d0d5dc",
            _ => "#9aa1af",
        };

        if !existing_node_ids.contains(&doc_node_id) {
            nodes.push(GalaxyNode {
                id: doc_node_id.clone(),
                label: doc_summary.title.clone(),
                category: doc_summary.category.clone(),
                kind: "document".to_string(),
                tier: "document".to_string(),
                x: dx,
                y: dy,
                z: dz,
                radius: 9.0,
                color: color.to_string(),
                description: format!("{} ({})", doc_summary.title, doc_summary.license),
                content: format!("{} (License: {}, Date: {})", doc_summary.title, doc_summary.license, doc_summary.retrieved_date),
                doc_id: Some(doc_summary.id.clone()),
                section_id: None,
                tags: vec![doc_summary.category.clone(), "document".to_string()],
            });
            existing_node_ids.insert(doc_node_id.clone());

            links.push(GalaxyLink {
                source_id: format!("category:{}", doc_summary.category),
                target_id: doc_node_id,
                relationship: "contains_document".to_string(),
                weight: 0.8,
            });
        }
    }

    // Field Note nodes and backlinks
    for (note_idx, note) in notes.iter().enumerate() {
        let note_id = format!("note:{}", note.id);
        let nr = (160.0 + (note_idx as f32 * 25.0)).min(1000.0);
        let n_angle = (note_idx as f32 * 1.37) % (std::f32::consts::PI * 2.0);

        if !existing_node_ids.contains(&note_id) {
            nodes.push(GalaxyNode {
                id: note_id.clone(),
                label: note.title.clone(),
                category: "notes".to_string(),
                kind: "note".to_string(),
                tier: "note".to_string(),
                x: n_angle.cos() * nr,
                y: 35.0 + (note_idx as f32 * 5.0) % 30.0,
                z: n_angle.sin() * nr,
                radius: 7.0,
                color: "#ff00bb".to_string(),
                description: note.title.clone(),
                content: note.content.clone(),
                doc_id: None,
                section_id: None,
                tags: vec!["note".to_string(), "field_note".to_string()],
            });
            existing_node_ids.insert(note_id.clone());

            for backlink in &note.links {
                let parts: Vec<&str> = backlink.split('#').collect();
                if let Some(target_doc) = parts.first() {
                    links.push(GalaxyLink {
                        source_id: note_id.clone(),
                        target_id: format!("doc:{}", target_doc),
                        relationship: "cites".to_string(),
                        weight: 0.9,
                    });
                }
            }
        }
    }

    let total = nodes.len();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let response_nodes = nodes.clone();
    let response_links = links.clone();

    Ok(Json(GalaxyGraphResponse {
        nodes,
        bodies: response_nodes,
        links,
        lines: response_links,
        total_nodes: total,
        timestamp: now,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateNodeRequest {
    pub label: String,
    pub tier: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub pos_x: Option<f32>,
    pub pos_y: Option<f32>,
    pub pos_z: Option<f32>,
}

pub async fn handle_create_node(
    State(state): State<ServerState>,
    Json(req): Json<CreateNodeRequest>,
) -> Result<Json<librarian_core::MemoryNode>, StatusCode> {
    let label = req.label.trim().to_string();
    if label.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let node_id = format!("mem:{}", uuid::Uuid::new_v4());
    let node = librarian_core::MemoryNode {
        id: node_id,
        tier: req.tier.unwrap_or_else(|| "memory".to_string()),
        label: label.clone(),
        content: req.content.unwrap_or_else(|| format!("Memory node: {}", label)),
        tags: req.tags.unwrap_or_else(|| vec!["memory".to_string(), "user_created".to_string()]),
        strength: 1.0,
        activation: 1.0,
        access_count: 0,
        created_at: now,
        last_accessed: now,
        pos_x: req.pos_x.unwrap_or_else(|| (now % 500) as f32 - 250.0),
        pos_y: req.pos_y.unwrap_or_else(|| ((now / 2) % 60) as f32 - 30.0),
        pos_z: req.pos_z.unwrap_or_else(|| ((now / 3) % 500) as f32 - 250.0),
        vel_x: 0.0,
        vel_y: 0.0,
        vel_z: 0.0,
    };
    state.db.insert_node(&node).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(node))
}
