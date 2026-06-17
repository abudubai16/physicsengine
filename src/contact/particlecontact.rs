use crate::basics::{ParticleEntry, ParticleStore, Vector};

/// A contact between two particles. Resolving a contact removes their interpenetration, and applies sufficient impulse to keep them apart. Collisions between particles can be modeled using restitution to make them bounce, or friction to prevent sliding. In this way, you can create realistic simulations of bouncing balls, or rolling wheels.
///
/// The stores the index of the first particle and the index of the second particle (if any). The second particle may be None, for contacts with the scenery.
type ParticleContactEntry = (usize, Option<usize>);

#[allow(dead_code)]
pub struct ParticleContact {
    particle_index: ParticleContactEntry,
    restitution: f32,
    contact_normal: Vector,
}

impl ParticleContact {
    pub fn new(
        particle_index: ParticleContactEntry,
        restitution: f32,
        contact_normal: Vector,
    ) -> Self {
        Self {
            particle_index,
            restitution,
            contact_normal,
        }
    }

    pub fn resolve(&self, particle_store: &mut ParticleStore, dt: f32) {
        self.resolve_velocity(particle_store, dt);
    }

    fn get_particle0<'a>(&self, particle_store: &'a mut ParticleStore) -> &'a mut ParticleEntry {
        match particle_store.get_particle_mut(self.particle_index.0) {
            Some(p0) => p0,
            None => panic!(
                "ParticleContact:Particle 0 at index {} not found",
                self.particle_index.0
            ),
        }
    }

    fn get_particle1<'a>(
        &self,
        particle_store: &'a mut ParticleStore,
    ) -> Option<&'a mut ParticleEntry> {
        match self.particle_index.1 {
            Some(index) => match particle_store.get_particle_mut(index) {
                Some(p1) => Some(p1),
                None => panic!("ParticleContact:Particle 1 at index {} not found", index),
            },
            None => None,
        }
    }

    pub fn calculate_separating_velocity(&self, particle_store: &mut ParticleStore) -> f32 {
        let particle0 = self.get_particle0(particle_store);
        let p0_velocity = particle0.velocity();

        match self.get_particle1(particle_store) {
            Some(particle1) => {
                let p1_velocity = particle1.velocity();
                (p0_velocity - p1_velocity) * self.contact_normal
            }
            None => p0_velocity * self.contact_normal,
        }
    }

    pub fn resolve_velocity(&self, particle_store: &mut ParticleStore, _: f32) {
        let separating_velocity = self.calculate_separating_velocity(particle_store);
        if separating_velocity > 0.0 {
            return;
        }

        let new_sep_velocity = -separating_velocity * self.restitution;
        let delta_velocity = new_sep_velocity - separating_velocity;

        let total_inverse_mass = {
            let p0 = self.get_particle0(particle_store);
            let inv0 = p0.inv_mass();
            let inv1 = self
                .get_particle1(particle_store)
                .map_or(0.0, |p1| p1.inv_mass());
            inv0 + inv1
        };

        if total_inverse_mass <= 0.0 {
            return;
        }

        let impulse = delta_velocity / total_inverse_mass;
        let impulse_per_imass = self.contact_normal * impulse;

        // Now take mutable borrows one at a time
        {
            let p0 = self.get_particle0(particle_store);
            let v0 = p0.velocity() + impulse_per_imass * p0.inv_mass();
            p0.set_velocity(v0);
        } // mutable borrow of p0 drops here

        if let Some(p1) = self.get_particle1(particle_store) {
            let v1 = p1.velocity() + impulse_per_imass * -p1.inv_mass();
            p1.set_velocity(v1);
        }
    }
}
