use super::generator::ParticleForceGenerator;
use crate::basics::{ParticleStore, Vector};

use libm;

use macroquad::prelude::draw_line;

/// A gravity generator applies a constant force to a particle, in the direction of gravity.
pub struct GravityGenerator {
    gravity: Vector,
}
impl GravityGenerator {
    pub fn new(gravity: Vector) -> Self {
        Self { gravity }
    }
}

impl ParticleForceGenerator for GravityGenerator {
    fn update_force(&self, particle_store: &mut ParticleStore, particle_index: usize, _: f32) {
        let particle = particle_store.get_particle_mut(particle_index);
        assert!(particle.inv_mass() > 0.0);
        particle.add_force(&(self.gravity * (1.0 / particle.inv_mass())));
    }
}

/// A particle spring generator applies a force to a particle that is connected to another particle by a spring.
pub struct ParticleSpringGenerator {
    other: usize, // Index to the other particle within the particle store
    spring_constant: f32,
    rest_length: f32,
}

impl ParticleSpringGenerator {
    pub fn new(other: usize, spring_constant: f32, rest_length: f32) -> Self {
        assert!(spring_constant > 0.0);
        assert!(rest_length >= 0.0);
        Self {
            other,
            spring_constant,
            rest_length,
        }
    }
}

impl ParticleForceGenerator for ParticleSpringGenerator {
    fn update_force(&self, particle_store: &mut ParticleStore, particle_index: usize, _: f32) {
        let p1_pos = particle_store.get_particle(self.other).position();
        let particle = particle_store.get_particle_mut(particle_index);
        let d = p1_pos - particle.position();
        let distance = d.magnitude();
        let norm_d = d.normalize();
        let force = norm_d * (distance - self.rest_length) * self.spring_constant;
        particle.add_force(&force);
    }
    fn draw_force(&self, particle_store: &ParticleStore, particle_index: usize) {
        let particle = particle_store.get_particle(particle_index);
        let other_particle = particle_store.get_particle(self.other);
        draw_line(
            particle.position().x,
            particle.position().y,
            other_particle.position().x,
            other_particle.position().y,
            2.0,
            macroquad::color::GREEN,
        );
    }
}

/// An anchored spring generator is like a particle spring generator, but one end is fixed in place.
pub struct AnchoredSpringGenerator {
    anchor: Vector,
    spring_constant: f32,
    rest_length: f32,
}

impl AnchoredSpringGenerator {
    pub fn new(anchor: Vector, spring_constant: f32, rest_length: f32) -> Self {
        assert!(spring_constant > 0.0);
        assert!(rest_length >= 0.0);
        Self {
            anchor,
            spring_constant,
            rest_length,
        }
    }
}

impl ParticleForceGenerator for AnchoredSpringGenerator {
    fn update_force(&self, particle_store: &mut ParticleStore, particle_index: usize, _: f32) {
        let particle = particle_store.get_particle_mut(particle_index);
        let d = particle.position() - self.anchor;
        let distance = d.magnitude();
        let norm_d = d.normalize();
        let force = norm_d * (self.rest_length - distance) * self.spring_constant;
        particle.add_force(&force);
    }
    fn draw_force(&self, particle_store: &ParticleStore, particle_index: usize) {
        let particle = particle_store.get_particle(particle_index);
        draw_line(
            particle.position().x,
            particle.position().y,
            self.anchor.x,
            self.anchor.y,
            2.0,
            macroquad::color::GREEN,
        );
    }
}

/// A bungee generator is like a spring generator, but it only applies a force if the particles are further apart than the rest length.
pub struct BungeeGenerator {
    other: usize,
    spring_constant: f32,
    rest_length: f32,
}

impl BungeeGenerator {
    pub fn new(particle: usize, spring_constant: f32, rest_length: f32) -> Self {
        Self {
            other: particle,
            spring_constant,
            rest_length,
        }
    }
}

impl ParticleForceGenerator for BungeeGenerator {
    fn update_force(&self, particle_store: &mut ParticleStore, particle_index: usize, _: f32) {
        let p1_pos = particle_store.get_particle(self.other).position();
        let particle = particle_store.get_particle_mut(particle_index);
        let d = particle.position() - p1_pos;
        let distance = d.magnitude();
        // println!("P0: {}, P1: {}", particle_index, self.other);
        if distance <= self.rest_length {
            return;
        }
        let norm_d = d.normalize();
        let force = norm_d * (self.rest_length - distance) * self.spring_constant;
        particle.add_force(&force);
    }
    fn draw_force(&self, particle_store: &ParticleStore, particle_index: usize) {
        let particle = particle_store.get_particle(particle_index);
        draw_line(
            particle.position().x,
            particle.position().y,
            particle_store.get_particle(self.other).position().x,
            particle_store.get_particle(self.other).position().y,
            2.0,
            macroquad::color::GREEN,
        );
    }
}

pub struct BouyancyGenerator {
    max_depth: f32,
    volume: f32,
    water_height: f32,
    liquid_density: f32,
}

impl BouyancyGenerator {
    pub fn new(max_depth: f32, volume: f32, water_height: f32, liquid_density: f32) -> Self {
        Self {
            max_depth,
            volume,
            water_height,
            liquid_density,
        }
    }
}

impl ParticleForceGenerator for BouyancyGenerator {
    fn update_force(&self, particle_store: &mut ParticleStore, particle_index: usize, _: f32) {
        let particle = particle_store.get_particle_mut(particle_index);
        let d = particle.position().y;
        if d >= self.water_height + self.max_depth {
            return;
        }
        if d <= self.water_height - self.max_depth {
            particle.add_force(&Vector {
                x: 0.0,
                y: self.liquid_density * self.volume,
                z: 0.0,
            });
            return;
        }
        let depth = self.water_height - d;
        particle.add_force(&Vector {
            x: 0.0,
            y: self.liquid_density * self.volume * depth / (2.0 * self.max_depth),
            z: 0.0,
        });
    }
}

pub struct StiffSpringGenerator {
    anchor: Vector,
    damping: f32,
    gamma: f32,
}

impl StiffSpringGenerator {
    pub fn new(anchor: Vector, spring_constant: f32, damping: f32) -> Self {
        assert!(spring_constant > 0.0);
        assert!(damping >= 0.0);
        let gamma = 0.5 * (4.0 * spring_constant - damping * damping).sqrt();
        Self {
            anchor,
            damping,
            gamma,
        }
    }
}

impl ParticleForceGenerator for StiffSpringGenerator {
    fn update_force(&self, particle_store: &mut ParticleStore, particle_index: usize, dt: f32) {
        let particle = particle_store.get_particle_mut(particle_index);
        if self.gamma == 0.0 {
            return;
        }
        let position = particle.position() - self.anchor;
        let c = position * (self.damping / (2.0 * self.gamma))
            + particle.velocity() * (1.0 / self.gamma);
        let mut target = position * libm::cos((self.gamma * dt) as f64) as f32
            + c * libm::sin((self.gamma * dt) as f64) as f32;
        target = target * libm::exp((-0.5 * dt * self.damping) as f64) as f32;
        let accel = (target - position) * (1.0 / libm::pow(dt as f64, 2.0)) as f32
            - particle.velocity() * (1.0 / dt);
        particle.add_force(&(accel * (1.0 / particle.inv_mass())));
    }
    fn draw_force(&self, particle_store: &ParticleStore, particle_index: usize) {
        let particle = particle_store.get_particle(particle_index);
        draw_line(
            particle.position().x,
            particle.position().y,
            self.anchor.x,
            self.anchor.y,
            2.0,
            macroquad::color::GREEN,
        );
    }
}
