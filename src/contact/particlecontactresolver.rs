use super::ParticleContact;
use crate::basics::ParticleStore;

pub struct ParticleContactResolver {
    iterations: u32,      // number of iterations allowed
    iterations_used: u32, // number of iterations used so far
}

impl ParticleContactResolver {
    pub fn new(iterations: u32) -> Self {
        Self {
            iterations,
            iterations_used: 0,
        }
    }

    pub fn set_iterations(&mut self, iterations: u32) {
        self.iterations = iterations;
    }

    pub fn resolve_contacts(
        &mut self,
        particle_contacts: &Vec<ParticleContact>,
        particle_store: &mut ParticleStore,
        dt: f32,
    ) {
        // If there are no contacts to resolve, return early
        if particle_contacts.is_empty() {
            return;
        }
        self.iterations_used = 0;
        while self.iterations_used < self.iterations {
            // Find the most severe contact based on the lowest seperating velocity, a negative
            // seperating velocity indicates that the particles are moving towards each other
            // and need to be resolved
            let mut max = f32::MAX;
            let mut max_index = particle_contacts.len();

            for (i, contact) in particle_contacts.iter().enumerate() {
                let sep_vel = contact.calculate_separating_velocity(particle_store);
                if sep_vel < max {
                    max = sep_vel;
                    max_index = i;
                }
            }

            if max_index == particle_contacts.len() {
                break;
            }
            particle_contacts[max_index].resolve(particle_store, dt);
            self.iterations_used += 1;
        }
    }
}
