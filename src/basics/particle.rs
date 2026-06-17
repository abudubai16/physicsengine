use super::vector::Vector;
use macroquad::prelude::Color;

#[allow(dead_code)]
pub type ParticleEntry = Box<dyn Particle>;

///
/// A trait representing a particle in a physics simulation. It defines the necessary properties and methods for implementation of a particle, including position, velocity, rotation, damping, mass and force accumulation.
///
pub trait Particle {
    fn position(&self) -> Vector;
    fn velocity(&self) -> Vector;
    fn rotation(&self) -> Vector;
    fn ang_vel(&self) -> Vector;
    fn damping(&self) -> f32;
    fn inv_mass(&self) -> f32;
    fn inv_inertia(&self) -> f32;
    fn force_accumulator(&self) -> Vector;

    fn set_velocity(&mut self, velocity: Vector);

    fn clear_accumulator(&mut self);
    fn integrate(&mut self, dt: f32);
    fn add_force(&mut self, force: &Vector);

    fn color(&self) -> Color;
    fn new() -> Self
    where
        Self: Sized;
}

/// The owner of the particles within the simulation, it handles the storage and provides ownership of
///the particles to other systems that need to access them. Responsible for handling the lifetime of
/// the particles and ensuring that they are properly managed and cleaned up when no longer needed.
#[allow(dead_code)]
pub struct ParticleStore {
    particles: Vec<ParticleEntry>,
}

impl ParticleStore {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn get_particle_mut(&mut self, index: usize) -> Option<&mut ParticleEntry> {
        self.particles.get_mut(index)
    }

    pub fn get_particle(&self, index: usize) -> Option<&ParticleEntry> {
        self.particles.get(index)
    }

    pub fn add_particle(&mut self, particle: ParticleEntry) {
        self.particles.push(particle);
    }

    pub fn remove_particle(&mut self, index: usize) -> Option<ParticleEntry> {
        if index < self.particles.len() {
            Some(self.particles.remove(index))
        } else {
            None
        }
    }
}
