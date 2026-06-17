use std::f32::consts;

use crate::basics::*;
use macroquad::prelude::{Color, RED};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Circle {
    pub pos: Vector,
    pub vel: Vector,
    pub rot: Vector,
    pub angular_vel: Vector,

    pub radius: f32,
    pub color: Color,
    pub mass_inv: f32,
    pub inertia_inv: f32,

    pub force_accum: Vector,
}

impl Circle {
    #[allow(dead_code)]
    fn check_collision(&self, other: &Circle) -> bool {
        if !self.simple_collision(other) {
            return false;
        }
        let dx = self.pos.x - other.pos.x;
        let dy = self.pos.y - other.pos.y;
        let distance_squared = dx * dx + dy * dy;
        let radius_sum = self.radius + other.radius;
        distance_squared < radius_sum * radius_sum
    }
}

impl Particle for Circle {
    fn position(&self) -> Vector {
        self.pos
    }
    fn set_velocity(&mut self, velocity: Vector) {
        self.vel = velocity;
    }
    fn velocity(&self) -> Vector {
        self.vel
    }
    fn rotation(&self) -> Vector {
        self.rot
    }
    fn ang_vel(&self) -> Vector {
        self.angular_vel
    }
    fn damping(&self) -> f32 {
        0.99
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
        self.rot = self.rot + self.angular_vel * dt;
        self.angular_vel = self.angular_vel * self.damping();
    }
    fn color(&self) -> Color {
        self.color
    }
    fn new() -> Self {
        Circle {
            pos: ZERO_VEC,
            vel: ZERO_VEC,
            rot: ZERO_VEC,
            angular_vel: ZERO_VEC,
            radius: 1.0,
            color: RED,
            mass_inv: 1.0 / consts::PI,
            inertia_inv: 1.0 / (0.5 * consts::PI),
            force_accum: ZERO_VEC,
        }
    }
}

impl BoundingBox for Circle {
    fn get_bounding_box(&self) -> (f32, f32, f32, f32) {
        (
            self.pos.x - self.radius,
            self.pos.x + self.radius,
            self.pos.y - self.radius,
            self.pos.y + self.radius,
        )
    }
}
