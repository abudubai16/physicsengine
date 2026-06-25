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
        self.iterations_used = 0;
        while self.iterations_used < self.iterations {
            let mut max = f32::MAX;
            let mut max_index = particle_contacts.len();

            // Find the contact with the largest closing velocity; if it's positive, we need to resolve it
            for (i, contact) in particle_contacts.iter().enumerate() {
                let sep_vel = contact.calculate_separating_velocity(particle_store);
                if sep_vel < max {
                    max = sep_vel; // update max
                    max_index = i; // update index together
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
