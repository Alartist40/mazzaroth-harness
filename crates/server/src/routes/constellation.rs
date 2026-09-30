use axum::response::Json;
use librarian_core::get_all_constellations;
use serde_json::json;

pub async fn handle_constellations_section() -> Json<serde_json::Value> {
    let constellations = get_all_constellations();
    let mut bodies = Vec::new();
    let mut lines = Vec::new();
    let mut catalog = Vec::new();

    for c in &constellations {
        let cluster_id = format!("constellation:{}:cluster", c.id);
        let season = c.season.clone().unwrap_or_else(|| "spring".to_string());
        let position = c.position.clone().unwrap_or_else(|| "north".to_string());
        let kind = c.kind.clone().unwrap_or_else(|| "asterism".to_string());
        let lore = c.lore.clone().unwrap_or_default();

        catalog.push(json!({
            "id": c.id,
            "label": c.label,
            "season": season,
            "position": position,
            "kind": kind,
            "lore": lore,
            "stars_count": c.stars.len(),
            "cluster_id": cluster_id,
        }));

        for s in &c.stars {
            let full_id = format!("constellation:{}:{}", c.id, s.id);
            let (px, py, pz) = (s.x, s.y, s.z);
            let orbit_radius = (px * px + pz * pz).sqrt();
            bodies.push(json!({
                "id": full_id,
                "label": s.name,
                "tier": "celestial",
                "constellation_id": c.id,
                "constellation_label": c.label,
                "season": season,
                "position": position,
                "kind": kind,
                "x": px, "y": py, "z": pz,
                "radius": 10.0,
                "color": if kind == "zodiac" { "#ffcc00" } else { "#00ffcc" },
                "luminosity": 0.95,
                "mass": 0.9,
                "orbit_radius": orbit_radius,
                "orbit_angle": 0.0,
                "content": format!("{} ({})\n{}\nSeason: {} · Sky: {}\nLore: {}", s.name, c.label, s.desc, season.to_uppercase(), position.to_uppercase(), lore),
                "tags": vec!["constellation", &c.id, &season, &position, &kind],
            }));
        }

        if let Some(first_star) = c.stars.first() {
            let (cx, cy, cz) = (first_star.x, first_star.y, first_star.z);
            let cluster_orbit_radius = (cx * cx + cz * cz).sqrt();
            bodies.push(json!({
                "id": cluster_id,
                "label": c.label.to_string(),
                "tier": "celestial",
                "constellation_id": c.id,
                "season": season,
                "position": position,
                "kind": kind,
                "x": cx, "y": cy, "z": cz,
                "radius": 14.0,
                "color": if kind == "zodiac" { "#ffaa00" } else { "#57d7ff" },
                "luminosity": 1.0,
                "mass": 1.0,
                "orbit_radius": cluster_orbit_radius,
                "orbit_angle": 0.0,
                "content": format!("Constellation: {}\nSeason: {} · Hemisphere: {}\n{}", c.label, season.to_uppercase(), position.to_uppercase(), lore),
                "tags": vec!["constellation", &c.id, &season, &position, &kind],
            }));
        }

        for (from_idx, to_idx) in &c.links {
            if *from_idx < c.stars.len() && *to_idx < c.stars.len() {
                lines.push(json!({
                    "source_id": format!("constellation:{}:{}", c.id, c.stars[*from_idx].id),
                    "target_id": format!("constellation:{}:{}", c.id, c.stars[*to_idx].id),
                    "constellation_id": c.id,
                    "weight": 0.85,
                    "relationship": "constellation_edge"
                }));
            }
        }
    }

    Json(json!({
        "bodies": bodies,
        "lines": lines,
        "catalog": catalog,
        "timestamp": 1727500000
    }))
}
