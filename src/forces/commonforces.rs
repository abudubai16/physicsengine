use super::generator::ForceGenerator;
use crate::basics::{ParticleEntry, Vector};

#[allow(dead_code)]
pub struct GravityGenerator {
    pub gravity: Vector,
}

impl ForceGenerator for GravityGenerator {
    fn update_force(&self, particle: &mut ParticleEntry, _: f32) {
        assert!(particle.inv_mass() > 0.0);
        particle.add_force(&(self.gravity * (1.0 / particle.inv_mass())));
    }
}
