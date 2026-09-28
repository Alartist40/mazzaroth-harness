use axum::response::Json;
use serde_json::json;
use crate::constellation::data::get_all_constellations;

pub async fn handle_constellations_section() -> Json<serde_json::Value> {
    let constellations = get_all_constellations();
    let mut bodies = Vec::new();
    let mut lines = Vec::new();

    for c in constellations {
        let cluster_id = format!("constellation:{}:cluster", c.id);
        
        for s in &c.stars {
            let full_id = format!("constellation:{}:{}", c.id, s.id);
            let (px, py, pz): (f32, f32, f32) = (s.x, s.y, s.z);
            let orbit_radius = (px * px + pz * pz).sqrt();
            bodies.push(json!({
                "id": full_id,
                "label": s.name,
                "tier": "celestial",
                "x": px, "y": py, "z": pz,
                "radius": 10.0,
                "color": { "r": 180, "g": 220, "b": 255 },
                "luminosity": 0.95,
                "mass": 0.9,
                "orbit_radius": orbit_radius,
                "orbit_angle": 0.0,
                "content": format!("{} ({})\n{}", s.name, c.label, s.desc),
                "tags": s.tags,
            }));
        }

        if let Some(first_star) = c.stars.first() {
            let (cx, cy, cz): (f32, f32, f32) = (first_star.x, first_star.y, first_star.z);
            let cluster_orbit_radius = (cx * cx + cz * cz).sqrt();
            bodies.push(json!({
                "id": cluster_id,
                "label": format!("🪐 {}", c.label),
                "tier": "celestial",
                "x": cx, "y": cy, "z": cz,
                "radius": 14.0,
                "color": { "r": 255, "g": 215, "b": 0 },
                "luminosity": 1.0,
                "mass": 1.0,
                "orbit_radius": cluster_orbit_radius,
                "orbit_angle": 0.0,
                "content": format!("Constellation Anchor: {}", c.label),
                "tags": vec!["constellation", c.id.as_str()],
            }));
        }

        for (from_idx, to_idx) in c.links {
            if from_idx < c.stars.len() && to_idx < c.stars.len() {
                lines.push(json!({
                    "source_id": format!("constellation:{}:{}", c.id, c.stars[from_idx].id),
                    "target_id": format!("constellation:{}:{}", c.id, c.stars[to_idx].id),
                    "weight": 0.85,
                    "relationship": "version_orbit"
                }));
            }
        }
    }

    Json(json!({
        "bodies": bodies,
        "lines": lines,
        "timestamp": 1727500000
    }))
}
