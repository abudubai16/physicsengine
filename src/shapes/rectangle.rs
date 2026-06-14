use macroquad::color::{Color, RED};

use crate::basics::*;

pub struct Rectangle {
    pub pos: Vector,
    pub vel: Vector,
    pub rot: Vector,
    pub angular_vel: Vector,
    pub width: f32,
    pub height: f32,
    pub mass: f32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn check_collision(&self, other: &Rectangle) -> bool {
        return self.simple_collision(other);
    }
}

impl Shape for Rectangle {
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

    fn color(&self) -> Color {
        RED
    }

    fn new() -> Self {
        Rectangle {
            pos: ZERO_VEC,
            vel: ZERO_VEC,
            rot: ZERO_VEC,
            angular_vel: ZERO_VEC,
            width: 1.0,
            height: 1.0,
            mass: 1.0,
        }
    }

    fn update_position(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }
    fn update_rotation(&mut self) {
        self.rot = self.rot + self.angular_vel;
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
