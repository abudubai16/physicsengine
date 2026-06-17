pub mod boundingbox;
pub mod particle;
pub mod vector;

pub use boundingbox::BoundingBox;
pub use particle::{Particle, ParticleEntry, ParticleStore};
pub use vector::Vector;

pub const G: Vector = Vector {
    x: 0.0,
    y: 9.81,
    z: 0.0,
};

pub const ZERO_VEC: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
