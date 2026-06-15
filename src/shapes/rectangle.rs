use macroquad::color::{Color, RED};

use crate::basics::*;

pub struct Rectangle {
    pub pos: Vector,
    pub vel: Vector,
    pub rot: Vector,
    pub angular_vel: Vector,

    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub mass_inv: f32,
    pub inertia_inv: f32,

    pub force_accum: Vector,
}

impl Rectangle {
    #[allow(dead_code)]
    fn check_collision(&self, other: &Rectangle) -> bool {
        return self.simple_collision(other);
    }
}

impl Particle for Rectangle {
    fn position(&self) -> Vector {
        self.pos
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
    fn clear_accumulator(&mut self) {
        self.force_accum = ZERO_VEC;
    }
    fn add_force(&mut self, force: &Vector) {
        self.force_accum = self.force_accum + *force;
    }
    fn force_accumulator(&self) -> Vector {
        self.force_accum
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
        Rectangle {
            pos: ZERO_VEC,
            vel: ZERO_VEC,
            rot: ZERO_VEC,
            angular_vel: ZERO_VEC,
            color: RED,
            width: 1.0,
            height: 1.0,
            mass_inv: 1.0,
            inertia_inv: 1.0,
            force_accum: ZERO_VEC,
        }
    }
}

impl BoundingBox for Rectangle {
    fn get_bounding_box(&self) -> (f32, f32, f32, f32) {
        (
            self.pos.x - self.width / 2.0,
            self.pos.x + self.width / 2.0,
            self.pos.y - self.height / 2.0,
            self.pos.y + self.height / 2.0,
        )
    }
}
