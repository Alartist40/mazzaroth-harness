use mazzaroth::{Camera3D, CelestialBody, CelestialPhysicsEngine, MemoryNode, MemoryTier};

#[test]
fn test_celestial_physics_and_projection() {
    let physics = CelestialPhysicsEngine::new();
    let node = MemoryNode::new("star_1", MemoryTier::Semantic, "Rust Skill", "High performance async");
    let mut bodies = vec![CelestialBody::from_node(&node)];

    bodies[0].x = 100.0;
    bodies[0].y = 0.0;
    bodies[0].z = 0.0;
    let initial_angle = bodies[0].orbit_angle;

    // Step physics
    physics.step(&mut bodies, &[], 0.1);

    // Orbit angle must advance
    assert!(bodies[0].orbit_angle > initial_angle, "Orbit angle must advance smoothly");

    // Camera perspective projection
    let camera = Camera3D::new(400.0, 300.0);
    let (sx, sy, depth) = camera.project(0.0, 0.0, 0.0);
    assert_eq!(sx, 400.0);
    assert_eq!(sy, 300.0);
    assert!(depth > 0.0);
}
