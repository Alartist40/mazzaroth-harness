use crate::galaxy::celestial::body::CelestialBody;
use crate::cognitive::node::AssociativeLink;
use std::collections::HashMap;

pub struct CelestialPhysicsEngine {
    pub gravity_constant: f32,
    pub damping: f32,
    pub link_spring_constant: f32,
    pub core_gravity_mass: f32,
}

impl Default for CelestialPhysicsEngine {
    fn default() -> Self {
        Self {
            gravity_constant: 5.0,
            damping: 0.98,
            link_spring_constant: 0.02,
            core_gravity_mass: 50.0,
        }
    }
}

impl CelestialPhysicsEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Advances the celestial physics simulation by dt seconds with O(1) link lookup
    pub fn step(&self, bodies: &mut [CelestialBody], links: &[AssociativeLink], dt: f32) {
        if bodies.is_empty() {
            return;
        }

        // Build fast ID -> Index lookup map with owned Strings to allow mutable borrow of bodies
        let id_map: HashMap<String, usize> = bodies
            .iter()
            .enumerate()
            .map(|(idx, b)| (b.id.clone(), idx))
            .collect();

        // 1. Orbital angular velocity advance
        for body in bodies.iter_mut() {
            body.orbit_angle += body.orbit_speed * dt * 5.0;
        }

        // 2. Link Spring Forces (Hebbian affinity maintains constellation coherence)
        for link in links {
            if let (Some(&s), Some(&t)) = (id_map.get(&link.source_id), id_map.get(&link.target_id)) {
                if s == t { continue; }
                let dx = bodies[t].x - bodies[s].x;
                let dy = bodies[t].y - bodies[s].y;
                let dz = bodies[t].z - bodies[s].z;
                let dist = (dx * dx + dy * dy + dz * dz + 1.0).sqrt();

                let rest_len = 50.0 * (1.1 - link.weight);
                let delta = dist - rest_len;
                let spring_force = self.link_spring_constant * link.weight * delta;

                let fx = (dx / dist) * spring_force * dt;
                let fy = (dy / dist) * spring_force * dt;
                let fz = (dz / dist) * spring_force * dt;

                bodies[s].vx += fx / bodies[s].mass;
                bodies[s].vy += fy / bodies[s].mass;
                bodies[s].vz += fz / bodies[s].mass;

                bodies[t].vx -= fx / bodies[t].mass;
                bodies[t].vy -= fy / bodies[t].mass;
                bodies[t].vz -= fz / bodies[t].mass;
            }
        }

        // 3. Integrate position with velocity damping
        for body in bodies.iter_mut() {
            body.vx *= self.damping;
            body.vy *= self.damping;
            body.vz *= self.damping;

            body.x += body.vx * dt * 10.0;
            body.y += body.vy * dt * 10.0;
            body.z += body.vz * dt * 10.0;
        }
    }
}
