use super::basics::*;
use super::contact::*;
use super::forces::*;

pub struct ParticleWorld {
    pub force_registry: ForceRegistry,
    pub contact_resolver: ParticleContactResolver,
    pub particle_store: ParticleStore,
    // pub particle_generator: ParticleContactGenerator,
    pub contacts: Vec<ParticleContact>,
    pub max_contacts: usize,

    pub constraints: Vec<Box<dyn ParticleLinkConstraint>>,
    pub particle_size: f32,

    pub screen_width: f32,
    pub screen_height: f32,
}

impl ParticleWorld {
    pub fn new(screen_width: f32, screen_height: f32, resolver_iterations: u32) -> Self {
        Self {
            force_registry: ForceRegistry::new(),
            contact_resolver: ParticleContactResolver::new(resolver_iterations),
            particle_store: ParticleStore::new(),
            contacts: Vec::new(),
            max_contacts: 100,
            constraints: Vec::new(),
            particle_size: 20.0,
            screen_width,
            screen_height,
        }
    }
    pub fn start_frame(&mut self) {
        for i in 0..self.particle_store.num_particles() {
            self.particle_store.get_particle_mut(i).clear_accumulator();
        }
    }

    pub fn generate_contacts(&mut self) -> u32 {
        let mut counter: u32 = 0;

        for i in 0..self.particle_store.num_particles() {
            let p0 = self.particle_store.get_particle(i);

            // Check for contacts with other particles
            for j in (i + 1)..self.particle_store.num_particles() {
                let p1 = self.particle_store.get_particle(j);
                let penetration =
                    (p1.position() - p0.position()).magnitude() - 2.0 * self.particle_size;
                if penetration <= 0.0 {
                    let contact = ParticleContact::new(
                        (i, Some(j)),
                        0.9,
                        (p1.position() - p0.position()).normalize(),
                        -penetration,
                        ContactType::ParticleParticle,
                    );
                    // println!(
                    //     "Generating a contact between particle {} and particle {}",
                    //     i, j
                    // );
                    self.contacts.push(contact);
                    counter += 1;
                }
            }

            // Check for contacts with the ground (y = 0)
            // if p0.position().y - self.particle_size >= self.screen_height {
            //     let contact = ParticleContact::new(
            //         (i, None),
            //         0.9,
            //         Vector {
            //             x: 0.0,
            //             y: -1.0,
            //             z: 0.0,
            //         },
            //         self.screen_height - (p0.position().y - self.particle_size),
            //     );
            //     println!("Generating a contact with the ground for particle {}", i);
            //     self.contacts.push(contact);
            //     counter += 1;
            // }

            //Check for constraint contacts
        }
        for (_, constraint) in self.constraints.iter().enumerate() {
            let some_contact = constraint.fill_contact(&self.particle_store);
            if let Some(contact) = some_contact {
                // println!("Generating a contact for constraint {}", i);
                self.contacts.push(contact);
                counter += 1;
            }
        }
        // println!("Total contacts generated: {} \n", counter);
        counter
    }

    pub fn integrate(&mut self, dt: f32) {
        for i in 0..self.particle_store.num_particles() {
            self.particle_store.get_particle_mut(i).integrate(dt);
        }
    }

    pub fn run_physics(&mut self, dt: f32) {
        self.force_registry
            .update_forces(&mut self.particle_store, dt);

        self.integrate(dt);

        let used_contacts = self.generate_contacts();

        self.contact_resolver.set_iterations(used_contacts * 2);

        self.contact_resolver
            .resolve_contacts(&self.contacts, &mut self.particle_store, dt);
    }

    pub fn draw(&self, highlight_forces: bool) {
        // Draw particles
        for i in 0..self.particle_store.num_particles() {
            let particle = self.particle_store.get_particle(i);
            let pos = particle.position();
            macroquad::shapes::draw_circle(pos.x, pos.y, self.particle_size, macroquad::color::RED);
        }

        // Draw constraints
        for constraint in self.constraints.iter() {
            let pair = constraint.particle_pair();
            let p1 = self.particle_store.get_particle(pair.0);
            let p2 = self.particle_store.get_particle(pair.1);
            macroquad::shapes::draw_line(
                p1.position().x,
                p1.position().y,
                p2.position().x,
                p2.position().y,
                2.0,
                macroquad::color::BLUE,
            );
        }

        // Draw force directions (optional)
        if !highlight_forces {
            return;
        }
        for i in 0..self.particle_store.num_particles() {
            let particle = self.particle_store.get_particle(i);
            let pos = particle.position();
            let force = particle.force_accumulator().normalize();

            macroquad::shapes::draw_line(
                pos.x,
                pos.y,
                pos.x + force.x * 20.0,
                pos.y + force.y * 20.0,
                20.0,
                macroquad::color::GREEN,
            );
        }
    }
}
