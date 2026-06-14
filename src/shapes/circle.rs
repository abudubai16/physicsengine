use std::f32::consts::PI;

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
    pub mass: f32,
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

impl Shape for Circle {
    fn position(&self) -> Vector {
        self.pos
    }

    fn velocity(&self) -> Vector {
        self.vel
    }

    fn color(&self) -> Color {
        self.color
    }

    fn rotation(&self) -> Vector {
        self.rot
    }

    fn ang_vel(&self) -> Vector {
        self.angular_vel
    }

    fn new() -> Self {
        Circle {
            pos: ZERO_VEC,
            vel: ZERO_VEC,
            rot: ZERO_VEC,
            angular_vel: ZERO_VEC,
            radius: 1.0,
            color: RED,
            mass: PI,
        }
    }

    fn update_position(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    fn update_rotation(&mut self) {
        self.rot.x += self.angular_vel.x;
        self.rot.y += self.angular_vel.y;
        self.rot.z += self.angular_vel.z;
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
