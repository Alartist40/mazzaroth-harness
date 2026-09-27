use crate::celestial::body::CelestialBody;
use crate::cognitive::node::AssociativeLink;

pub struct CelestialPhysicsEngine {
    pub gravity_constant: f32,
    pub damping: f32,
    pub link_spring_constant: f32,
    pub core_gravity_mass: f32,
}

impl Default for CelestialPhysicsEngine {
    fn default() -> Self {
        Self {
            gravity_constant: 15.0,
            damping: 0.96,
            link_spring_constant: 0.08,
            core_gravity_mass: 100.0,
        }
    }
}

impl CelestialPhysicsEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Advances the celestial physics simulation by dt seconds
    pub fn step(&self, bodies: &mut [CelestialBody], links: &[AssociativeLink], dt: f32) {
        if bodies.is_empty() {
            return;
        }


        // 1. Orbital rotation and central core gravitational pull
        for body in bodies.iter_mut() {
            // Apply orbital angular velocity
            body.orbit_angle += body.orbit_speed * dt * 20.0;
            
            // Central gravitational attraction towards origin (0, 0, 0)
            let dist_sq = body.x * body.x + body.y * body.y + body.z * body.z + 100.0;
            let dist = dist_sq.sqrt();
            let force_mag = (self.gravity_constant * self.core_gravity_mass * body.mass) / dist_sq;

            let dir_x = -body.x / dist;
            let dir_y = -body.y / dist;
            let dir_z = -body.z / dist;

            body.vx += dir_x * force_mag * dt;
            body.vy += dir_y * force_mag * dt;
            body.vz += dir_z * force_mag * dt;

            // Orbital tangential velocity
            let tangent_x = -body.z / dist;
            let tangent_z = body.x / dist;
            body.vx += tangent_x * body.orbit_speed * 15.0 * dt;
            body.vz += tangent_z * body.orbit_speed * 15.0 * dt;
        }

        // 2. Link Spring Forces (Hebbian affinity attracts connected stars into constellations)
        for link in links {
            let mut src_idx = None;
            let mut tgt_idx = None;

            for (i, b) in bodies.iter().enumerate() {
                if b.id == link.source_id {
                    src_idx = Some(i);
                }
                if b.id == link.target_id {
                    tgt_idx = Some(i);
                }
            }

            if let (Some(s), Some(t)) = (src_idx, tgt_idx) {
                let dx = bodies[t].x - bodies[s].x;
                let dy = bodies[t].y - bodies[s].y;
                let dz = bodies[t].z - bodies[s].z;
                let dist = (dx * dx + dy * dy + dz * dz + 1.0).sqrt();

                // Target rest length depends on inverted link weight
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

        // 3. Integrate position and apply velocity damping
        for body in bodies.iter_mut() {
            body.vx *= self.damping;
            body.vy *= self.damping;
            body.vz *= self.damping;

            body.x += body.vx * dt * 50.0;
            body.y += body.vy * dt * 50.0;
            body.z += body.vz * dt * 50.0;
        }
    }
}
