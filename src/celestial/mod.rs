pub mod body;
pub mod physics;
pub mod projection;
pub mod constellation;

pub use body::{CelestialBody, SpectralColor};
pub use physics::CelestialPhysicsEngine;
pub use projection::Camera3D;
pub use constellation::{build_constellation_lines, ConstellationCluster, ConstellationLine};
