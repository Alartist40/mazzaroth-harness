use mazzaroth::{CognitiveDecayEngine, MemoryNode, MemoryTier};

#[test]
fn test_cognitive_tiers_and_decay() {
    let decay_engine = CognitiveDecayEngine::new();

    let mut working_node = MemoryNode::new("work_1", MemoryTier::Working, "Recent turn", "User said hello");
    let mut celestial_node = MemoryNode::new("core_1", MemoryTier::Celestial, "Core Identity", "System Sovereign");

    assert_eq!(working_node.strength, 0.4);
    assert_eq!(celestial_node.strength, 1.0);

    let future_time = working_node.last_accessed + 7200; // 2 hours later
    decay_engine.apply_temporal_decay(&mut working_node, future_time);
    decay_engine.apply_temporal_decay(&mut celestial_node, future_time);

    // Working memory decays activation and strength
    assert!(working_node.activation < 0.5);
    assert!(working_node.strength < 0.4);

    // Celestial memory preserves immutable strength 1.0
    assert_eq!(celestial_node.strength, 1.0);
}
