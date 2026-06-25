use crate::basics::{ParticleContactEntry, ParticleStore, Vector, ZERO_VEC};

/// A contact between two particles. Resolving a contact removes their interpenetration, and applies sufficient impulse to keep them apart. Collisions between particles can be modeled using restitution to make them bounce, or friction to prevent sliding. In this way, you can create realistic simulations of bouncing balls, or rolling wheels.
///
/// The stores the index of the first particle and the index of the second particle (if any). The second particle may be None, for contacts with the scenery.

#[derive(Debug, Clone)]
pub enum ContactType {
    ParticleParticle,
    ParticleScenery,
    Constraint,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParticleContact {
    particle_index: ParticleContactEntry,
    restitution: f32,
    contact_normal: Vector,
    penetration: f32,
    contact_type: ContactType,
}

impl ParticleContact {
    pub fn new(
        particle_index: ParticleContactEntry,
        restitution: f32,
        contact_normal: Vector,
        penetration: f32,
        contact_type: ContactType,
    ) -> Self {
        assert!(
            restitution >= 0.0 && restitution <= 1.0,
            "Restitution must be between 0 and 1"
        );
        assert!(
            (contact_normal.magnitude() - 1.0).abs() < 1e-6,
            "Contact normal is {}, must be a unit vector, particle_index : {:?}",
            contact_normal.magnitude(),
            particle_index
        );
        Self {
            particle_index,
            restitution,
            contact_normal,
            penetration,
            contact_type,
        }
    }

    pub fn resolve(&self, particle_store: &mut ParticleStore, dt: f32) {
        self.resolve_velocity(particle_store, dt);
        self.resolve_interpenetration(particle_store, dt);
    }

    pub fn calculate_separating_velocity(&self, particle_store: &mut ParticleStore) -> f32 {
        let p0_velocity = particle_store
            .get_particle(self.particle_index.0)
            .velocity();

        match self.particle_index.1 {
            Some(index) => {
                let p1_velocity = particle_store.get_particle(index).velocity();
                (p0_velocity - p1_velocity) * self.contact_normal
            }
            None => p0_velocity * self.contact_normal,
        }
    }

    pub fn resolve_velocity(&self, particle_store: &mut ParticleStore, dt: f32) {
        let separating_velocity = self.calculate_separating_velocity(particle_store);
        if separating_velocity > 0.0 {
            return;
        }
        let mut new_sep_velocity = -separating_velocity * self.restitution;

        let acc_caused_velocity = {
            let p0_acc = particle_store
                .get_particle(self.particle_index.0)
                .acceleration();

            let p1_acc = self.particle_index.1.map_or(ZERO_VEC, |index| {
                particle_store.get_particle(index).acceleration()
            });

            (p0_acc - p1_acc) * self.contact_normal * dt
        };
        if acc_caused_velocity < 0.0 {
            new_sep_velocity += acc_caused_velocity * self.restitution;
            new_sep_velocity = new_sep_velocity.max(0.0);
        }
        //
        let delta_velocity = new_sep_velocity - separating_velocity;

        let total_inverse_mass = {
            let p0 = particle_store.get_particle(self.particle_index.0);
            let inv0 = p0.inv_mass();
            let inv1 = self
                .particle_index
                .1
                .map_or(0.0, |index| particle_store.get_particle(index).inv_mass());
            inv0 + inv1
        };

        if total_inverse_mass <= 0.0 {
            return;
        }

        let impulse = delta_velocity / total_inverse_mass;
        let impulse_per_imass = self.contact_normal * impulse;

        // Now take mutable borrows one at a time
        {
            let p0 = particle_store.get_particle_mut(self.particle_index.0);
            let v0 = p0.velocity() + impulse_per_imass * p0.inv_mass();
            p0.set_velocity(v0);
        } // mutable borrow of p0 drops here

        match self.particle_index.1 {
            Some(index) => {
                let p1 = particle_store.get_particle_mut(index);
                let v1 = p1.velocity() + impulse_per_imass * -p1.inv_mass();
                p1.set_velocity(v1);
            }
            _ => {}
        }
    }

    pub fn resolve_interpenetration(&self, particle_store: &mut ParticleStore, _: f32) {
        if self.penetration <= 0.0 {
            return;
        }

        let total_inverse_mass = {
            let inv0 = particle_store
                .get_particle(self.particle_index.0)
                .inv_mass();
            let inv1 = self
                .particle_index
                .1
                .map_or(0.0, |index| particle_store.get_particle(index).inv_mass());
            inv0 + inv1
        };

        if total_inverse_mass <= 0.0 {
            return;
        }

        let move_per_imass = self.contact_normal * (-self.penetration / total_inverse_mass);

        // Now take mutable borrows one at a time
        {
            let p0 = particle_store.get_particle_mut(self.particle_index.0);
            let new_position = p0.position() + move_per_imass * p0.inv_mass();
            p0.set_position(new_position);
        } // mutable borrow of p0 drops here

        if let Some(index) = self.particle_index.1 {
            let p1 = particle_store.get_particle_mut(index);
            let new_position = p1.position() + move_per_imass * -p1.inv_mass();
            p1.set_position(new_position);
        }
    }
}
