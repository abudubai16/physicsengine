use super::{ContactType, ParticleContact};
use crate::basics::{ParticlePair, ParticleStore};

#[allow(unused)]
pub trait ParticleLinkConstraint {
    fn particle_pair(&self) -> ParticlePair;
    fn current_length(&self, particle_store: &ParticleStore) -> f32;
    fn fill_contact(&self, particle_store: &ParticleStore) -> Option<ParticleContact>;
}

pub struct ParticleCable {
    particles: ParticlePair,
    max_length: f32,
    restitution: f32,
}

impl ParticleCable {
    pub fn new(particles: ParticlePair, max_length: f32, restitution: f32) -> Self {
        Self {
            particles,
            max_length,
            restitution,
        }
    }
}

impl ParticleLinkConstraint for ParticleCable {
    fn particle_pair(&self) -> ParticlePair {
        self.particles
    }
    fn current_length(&self, particle_store: &ParticleStore) -> f32 {
        let p1 = particle_store.get_particle(self.particles.0);
        let p2 = particle_store.get_particle(self.particles.1);
        (p2.position() - p1.position()).magnitude()
    }

    fn fill_contact(&self, particle_store: &ParticleStore) -> Option<ParticleContact> {
        let length = self.current_length(particle_store);
        if length < self.max_length {
            return None;
        }
        let p1 = particle_store.get_particle(self.particles.0);
        let p2 = particle_store.get_particle(self.particles.1);
        let normal = (p2.position() - p1.position()).normalize();

        Some(ParticleContact::new(
            (self.particles.0, Some(self.particles.1)),
            self.restitution,
            normal,
            length - self.max_length,
            ContactType::Constraint,
        ))
    }
}

pub struct ParticleRod {
    particles: ParticlePair,
    length: f32,
}

impl ParticleRod {
    pub fn new(particles: ParticlePair, length: f32) -> Self {
        Self { particles, length }
    }
}

impl ParticleLinkConstraint for ParticleRod {
    fn particle_pair(&self) -> ParticlePair {
        self.particles
    }
    fn current_length(&self, particle_store: &ParticleStore) -> f32 {
        let p1 = particle_store.get_particle(self.particles.0);
        let p2 = particle_store.get_particle(self.particles.1);
        (p2.position() - p1.position()).magnitude()
    }
    fn fill_contact(&self, particle_store: &ParticleStore) -> Option<ParticleContact> {
        let length = self.current_length(particle_store);
        if length == self.length {
            return None;
        }
        let p1 = particle_store.get_particle(self.particles.0);
        let p2 = particle_store.get_particle(self.particles.1);
        let normal = (p2.position() - p1.position()).normalize();

        Some(ParticleContact::new(
            (self.particles.0, Some(self.particles.1)),
            1.0,
            normal,
            length - self.length,
            ContactType::Constraint,
        ))
    }
}
