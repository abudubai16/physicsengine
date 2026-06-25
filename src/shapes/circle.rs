use std::f32::consts;

use crate::basics::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ParticleCircle {
    pub pos: Vector,
    pub vel: Vector,
    pub mass_inv: f32,
    pub inertia_inv: f32,

    pub force_accum: Vector,
}

impl Particle for ParticleCircle {
    fn position(&self) -> Vector {
        self.pos
    }
    fn set_position(&mut self, position: Vector) {
        self.pos = position;
    }
    fn velocity(&self) -> Vector {
        self.vel
    }
    fn set_velocity(&mut self, velocity: Vector) {
        self.vel = velocity;
    }

    fn damping(&self) -> f32 {
        1.0
    }
    fn inv_mass(&self) -> f32 {
        self.mass_inv
    }
    fn inv_inertia(&self) -> f32 {
        self.inertia_inv
    }
    fn force_accumulator(&self) -> Vector {
        self.force_accum
    }
    fn clear_accumulator(&mut self) {
        self.force_accum = ZERO_VEC;
    }
    fn add_force(&mut self, force: &Vector) {
        self.force_accum = self.force_accum + *force;
    }
    fn integrate(&mut self, dt: f32) {
        self.pos = self.pos + self.vel * dt;
        self.vel = self.vel + self.force_accum * self.mass_inv * dt;
        self.vel = self.vel * self.damping();
    }
    fn new() -> Self {
        ParticleCircle {
            pos: ZERO_VEC,
            vel: ZERO_VEC,
            mass_inv: 1.0 / consts::PI,
            inertia_inv: 1.0 / (0.5 * consts::PI),
            force_accum: ZERO_VEC,
        }
    }
}
