pub mod cognitive;
pub mod store;
pub mod engine;
pub mod server;

#[path = "../galaxy/src/lib.rs"]
pub mod galaxy;

#[path = "../constellation/src/lib.rs"]
pub mod constellation;

pub use cognitive::{AssociativeLink, CognitiveDecayEngine, MemoryNode, MemoryTier};
pub use galaxy::celestial::{build_constellation_lines, Camera3D, CelestialBody, CelestialPhysicsEngine, ConstellationLine};
pub use galaxy::corpus::CorpusImporter;
pub use store::MazzarothStore;
pub use engine::{CelestialGalaxyState, MazzarothEngine};
pub use server::{create_router, ServerState};
