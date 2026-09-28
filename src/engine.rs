use crate::celestial::{build_constellation_lines, CelestialBody, CelestialPhysicsEngine, ConstellationLine};
use crate::cognitive::{AssociativeLink, CognitiveDecayEngine, MemoryNode, MemoryTier};
use crate::store::MazzarothStore;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelestialGalaxyState {
    pub bodies: Vec<CelestialBody>,
    pub lines: Vec<ConstellationLine>,
    pub timestamp: i64,
}

#[derive(Clone)]
pub struct MazzarothEngine {
    pub store: MazzarothStore,
    pub decay: Arc<CognitiveDecayEngine>,
    pub physics: Arc<CelestialPhysicsEngine>,
    pub bodies: Arc<Mutex<Vec<CelestialBody>>>,
}

impl MazzarothEngine {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let store = MazzarothStore::open(path)?;
        let engine = Self {
            store,
            decay: Arc::new(CognitiveDecayEngine::default()),
            physics: Arc::new(CelestialPhysicsEngine::default()),
            bodies: Arc::new(Mutex::new(Vec::new())),
        };
        engine.ensure_celestial_anchors()?;
        engine.sync_celestial_bodies()?;
        Ok(engine)
    }

    pub fn in_memory() -> Result<Self> {
        let store = MazzarothStore::open_in_memory()?;
        let engine = Self {
            store,
            decay: Arc::new(CognitiveDecayEngine::default()),
            physics: Arc::new(CelestialPhysicsEngine::default()),
            bodies: Arc::new(Mutex::new(Vec::new())),
        };
        engine.ensure_celestial_anchors()?;
        engine.sync_celestial_bodies()?;
        Ok(engine)
    }

    /// Seeds foundational immutable identity anchors
    pub fn ensure_celestial_anchors(&self) -> Result<()> {
        if self.store.count_nodes()? == 0 {
            let core = MemoryNode {
                id: "celestial:core:identity".to_string(),
                tier: MemoryTier::Celestial,
                label: "Core Identity".to_string(),
                content: "Mazzaroth Autonomous AI & Cognitive Entity. Local-first, sovereign intelligence.".to_string(),
                tags: vec!["identity".into(), "axiom".into(), "sovereign".into()],
                strength: 1.0,
                activation: 1.0,
                access_count: 1,
                created_at: Self::now(),
                last_accessed: Self::now(),
                pos_x: 0.0,
                pos_y: 0.0,
                pos_z: 0.0,
                vel_x: 0.0,
                vel_y: 0.0,
                vel_z: 0.0,
            };
            self.store.insert_node(&core)?;

            let rules = MemoryNode {
                id: "celestial:axiom:local_first".to_string(),
                tier: MemoryTier::Celestial,
                label: "Local First Axiom".to_string(),
                content: "Compute locally, protect sovereignty, preserve privacy.".to_string(),
                tags: vec!["axiom".into(), "rule".into()],
                strength: 1.0,
                activation: 0.9,
                access_count: 1,
                created_at: Self::now(),
                last_accessed: Self::now(),
                pos_x: 60.0,
                pos_y: 20.0,
                pos_z: -30.0,
                vel_x: 0.0,
                vel_y: 0.0,
                vel_z: 0.0,
            };
            self.store.insert_node(&rules)?;

            self.store.insert_link(&AssociativeLink {
                source_id: core.id.clone(),
                target_id: rules.id.clone(),
                weight: 0.95,
                relationship: "anchors".to_string(),
                created_at: Self::now(),
                last_reinforced: Self::now(),
            })?;
        }
        Ok(())
    }

    /// Syncs bodies from database into memory simulation
    pub fn sync_celestial_bodies(&self) -> Result<()> {
        let nodes = self.store.get_all_nodes()?;
        let mut bodies = Vec::new();
        for node in nodes {
            bodies.push(CelestialBody::from_node(&node));
        }
        let mut guard = self.bodies.lock().unwrap();
        *guard = bodies;
        Ok(())
    }

    /// Ingests a new memory turn / sensory perception
    pub fn ingest(&self, label: &str, content: &str, tier: MemoryTier, tags: Vec<String>) -> Result<MemoryNode> {
        let id = format!("{}:{}", tier.as_str(), uuid::Uuid::new_v4().to_string().replace('-', ""));
        let mut node = MemoryNode::new(&id, tier, label, content);
        node.tags = tags;

        // Position nodes along 4-arm spiral with matching 0.003 twist and disc wave
        let arms = 4.0;
        let arm = (rand_simple() * arms).floor().min(3.0);
        let arm_offset = (arm / arms) * std::f32::consts::PI * 2.0;
        let radius = match tier {
            MemoryTier::Celestial => 0.0,
            MemoryTier::Semantic => 100.0 + rand_simple() * 150.0,
            MemoryTier::Episodic => 250.0 + rand_simple() * 250.0,
            MemoryTier::Working => 500.0 + rand_simple() * 300.0,
        };
        let spiral_angle = arm_offset + (radius * 0.003);
        let fuzz = (rand_simple() - 0.5) * 20.0;

        node.pos_x = spiral_angle.cos() * radius + fuzz;
        node.pos_y = (radius * 0.01).sin() * 20.0 + (rand_simple() - 0.5) * 10.0;
        node.pos_z = spiral_angle.sin() * radius + fuzz;

        self.store.insert_node(&node)?;

        // Connect to core identity if relevant
        if tier != MemoryTier::Celestial {
            self.store.insert_link(&AssociativeLink {
                source_id: "celestial:core:identity".to_string(),
                target_id: node.id.clone(),
                weight: 0.5,
                relationship: "orbits".to_string(),
                created_at: Self::now(),
                last_reinforced: Self::now(),
            })?;
        }

        self.sync_celestial_bodies()?;
        Ok(node)
    }

    /// Searches and recalls memories using full-text BM25 index & Hebbian reinforcement
    pub fn recall(&self, query: &str, limit: usize) -> Result<Vec<MemoryNode>> {
        let nodes = self.store.search_fts(query, limit)?;
        let now = Self::now();

        // Reinforce accessed nodes
        for node in &nodes {
            if let Ok(Some(mut n)) = self.store.get_node(&node.id) {
                n.access_count += 1;
                n.last_accessed = now;
                n.activation = (n.activation + 0.3).min(1.0);
                let _ = self.store.insert_node(&n);
            }
        }

        // Reinforce associative links between co-recalled memories
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let link = AssociativeLink {
                    source_id: nodes[i].id.clone(),
                    target_id: nodes[j].id.clone(),
                    weight: 0.7,
                    relationship: "co_recalled".to_string(),
                    created_at: now,
                    last_reinforced: now,
                };
                let _ = self.store.insert_link(&link);
            }
        }

        Ok(nodes)
    }

    /// Runs a simulation physics step
    pub fn step_physics(&self, dt: f32) -> Result<CelestialGalaxyState> {
        let links = self.store.get_all_links()?;
        let mut bodies_guard = self.bodies.lock().unwrap();

        self.physics.step(&mut bodies_guard, &links, dt);

        let lines = build_constellation_lines(&links);
        Ok(CelestialGalaxyState {
            bodies: bodies_guard.clone(),
            lines,
            timestamp: Self::now(),
        })
    }

    pub fn get_galaxy_state(&self) -> Result<CelestialGalaxyState> {
        let links = self.store.get_all_links()?;
        let bodies_guard = self.bodies.lock().unwrap();
        let lines = build_constellation_lines(&links);
        Ok(CelestialGalaxyState {
            bodies: bodies_guard.clone(),
            lines,
            timestamp: Self::now(),
        })
    }

    fn now() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}

static mut SEED: u64 = 123456789;
fn rand_simple() -> f32 {
    unsafe {
        SEED = (SEED.wrapping_mul(6364136223846793005)).wrapping_add(1442695040888963407);
        ((SEED >> 32) as u32 as f32) / (u32::MAX as f32)
    }
}
