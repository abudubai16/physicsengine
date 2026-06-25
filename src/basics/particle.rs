use super::vector::Vector;

/// A type that represents a struct that implements the Particle trait, it is used to store particles in the ParticleStore. It is a boxed trait object, which allows for dynamic dispatch and flexibility in the types of particles that can be stored.
pub type ParticleEntry = Box<dyn Particle>;

/// A pair of particle indices, representing a contact between two particles. The first index is always present, while the second index may be None for contacts with the scenery.
pub type ParticleContactEntry = (usize, Option<usize>);

/// A pair of particle indices, representing a contact between two particles. Both indices are always present, and must be valid indices in the ParticleStore.
pub type ParticlePair = (usize, usize);

///
/// A trait representing a particle in a physics simulation. It defines the necessary properties and methods for implementation of a particle, including position, velocity, rotation, damping, mass and force accumulation.
///
pub trait Particle {
    fn position(&self) -> Vector;
    fn velocity(&self) -> Vector;
    fn damping(&self) -> f32;
    fn inv_mass(&self) -> f32;
    fn inv_inertia(&self) -> f32;
    fn force_accumulator(&self) -> Vector;
    fn acceleration(&self) -> Vector {
        self.force_accumulator() * self.inv_mass()
    }

    fn set_velocity(&mut self, velocity: Vector);
    fn set_position(&mut self, position: Vector);

    fn clear_accumulator(&mut self);
    fn integrate(&mut self, dt: f32);
    fn add_force(&mut self, force: &Vector);

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

    pub fn num_particles(&self) -> usize {
        self.particles.len()
    }

    pub fn get_particle_mut(&mut self, index: usize) -> &mut ParticleEntry {
        assert!(index < self.particles.len(), "Index out of bounds");
        self.particles.get_mut(index).unwrap()
    }

    pub fn get_particle(&self, index: usize) -> &ParticleEntry {
        assert!(index < self.particles.len(), "Index out of bounds");
        self.particles.get(index).unwrap()
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
