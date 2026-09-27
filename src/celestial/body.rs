use crate::cognitive::node::{MemoryNode, MemoryTier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelestialBody {
    pub id: String,
    pub label: String,
    pub content: String,
    pub tags: Vec<String>,
    pub tier: MemoryTier,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub mass: f32,
    pub luminosity: f32,
    pub radius: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_angle: f32,
    pub color: SpectralColor,
    pub created_at: i64,
    pub last_accessed: i64,
}

impl CelestialBody {
    pub fn from_node(node: &MemoryNode) -> Self {
        let (color, base_radius, orbit_speed) = match node.tier {
            MemoryTier::Celestial => (
                SpectralColor { r: 255, g: 215, b: 0, a: 255 }, // Golden Core Star
                12.0,
                0.002,
            ),
            MemoryTier::Semantic => (
                SpectralColor { r: 56, g: 189, b: 248, a: 220 }, // Cyan Star
                8.0,
                0.015,
            ),
            MemoryTier::Episodic => (
                SpectralColor { r: 168, g: 85, b: 247, a: 200 }, // Violet Planet
                5.5,
                0.025,
            ),
            MemoryTier::Working => (
                SpectralColor { r: 251, g: 146, b: 60, a: 180 }, // Amber Comet
                4.0,
                0.040,
            ),
        };

        let orbit_radius = (node.pos_x * node.pos_x + node.pos_y * node.pos_y + node.pos_z * node.pos_z).sqrt().max(20.0);

        Self {
            id: node.id.clone(),
            label: node.label.clone(),
            content: node.content.clone(),
            tags: node.tags.clone(),
            tier: node.tier,
            x: node.pos_x,
            y: node.pos_y,
            z: node.pos_z,
            vx: node.vel_x,
            vy: node.vel_y,
            vz: node.vel_z,
            mass: node.strength.max(0.1),
            luminosity: node.activation.max(0.1),
            radius: base_radius * node.strength.max(0.3),
            orbit_radius,
            orbit_speed,
            orbit_angle: 0.0,
            color,
            created_at: node.created_at,
            last_accessed: node.last_accessed,
        }
    }
}
