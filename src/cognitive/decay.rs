use crate::cognitive::node::{AssociativeLink, MemoryNode, MemoryTier};

pub struct CognitiveDecayEngine {
    pub decay_half_life_secs: f32, // Time for activation to decay by 50%
    pub hebbian_learning_rate: f32,
    pub min_retention_threshold: f32,
}

impl Default for CognitiveDecayEngine {
    fn default() -> Self {
        Self {
            decay_half_life_secs: 3600.0, // 1 hour default half life
            hebbian_learning_rate: 0.15,
            min_retention_threshold: 0.1,
        }
    }
}

impl CognitiveDecayEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies Ebbinghaus forgetting curve decay to a node's activation and strength
    pub fn apply_temporal_decay(&self, node: &mut MemoryNode, current_time: i64) {
        if node.tier == MemoryTier::Celestial {
            node.strength = 1.0;
            node.activation = (node.activation * 0.95).max(0.5);
            return;
        }

        let elapsed = (current_time - node.last_accessed).max(0) as f32;
        let decay_factor = (-elapsed / self.decay_half_life_secs).exp();

        // Activation drops quickly with time
        node.activation = (node.activation * decay_factor).max(0.01);

        // Strength (retention) decays much slower based on tier
        let strength_decay_rate = match node.tier {
            MemoryTier::Working => 0.005,
            MemoryTier::Episodic => 0.0005,
            MemoryTier::Semantic => 0.00005,
            MemoryTier::Celestial => 0.0,
        };

        let strength_loss = elapsed * strength_decay_rate;
        node.strength = (node.strength - strength_loss).max(self.min_retention_threshold);
    }

    /// Reinforces Hebbian link weight when two memories fire/are recalled together
    pub fn reinforce_hebbian_link(&self, link: &mut AssociativeLink, current_time: i64) {
        link.weight = (link.weight + self.hebbian_learning_rate * (1.0 - link.weight)).min(1.0);
        link.last_reinforced = current_time;
    }

    /// Decays unused synaptic link weights
    pub fn decay_hebbian_link(&self, link: &mut AssociativeLink, current_time: i64) {
        let elapsed = (current_time - link.last_reinforced).max(0) as f32;
        let decay_factor = (-elapsed / (self.decay_half_life_secs * 4.0)).exp();
        link.weight = (link.weight * decay_factor).max(0.01);
    }
}
