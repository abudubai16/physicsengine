/// This module contains the `ParticleContact` struct and its associated methods for resolving particle contacts in
/// a physics simulation.
///
/// The flow order is as follows:
/// 1. ParticleCollisionDetector detects collisions and creates ParticleContact instances.
/// 2. ParticleContactResolver resolves the contacts by adjusting particle velocities and positions.
///
pub mod constraints;
pub mod particlecontact;
pub mod particlecontactresolver;

pub use constraints::{ParticleCable, ParticleLinkConstraint, ParticleRod};
pub use particlecontact::*;
pub use particlecontactresolver::ParticleContactResolver;
