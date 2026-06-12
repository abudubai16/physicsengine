use macroquad::prelude::Color;

use std::{
    f32,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    pub angle: f32,
    pub angular_velocity: f32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[allow(dead_code)]
pub trait Shape {
    fn position(&self) -> Vector;
    fn velocity(&self) -> Velocity;
    fn rotation(&self) -> Rotation;
    fn color(&self) -> Color;

    fn new() -> Self
    where
        Self: Sized;

    fn update_position(&mut self);
    fn update_rotation(&mut self);
}

#[allow(dead_code)]
pub trait BoundingBox {
    // (left, right, top, bottom)
    fn get_bounding_box(&self) -> (f32, f32, f32, f32);

    fn simple_collision(&self, other: &Self) -> bool {
        let (left1, right1, top1, bottom1) = self.get_bounding_box();
        let (left2, right2, top2, bottom2) = other.get_bounding_box();

        !(right1 < left2 || left1 > right2 || bottom1 < top2 || top1 > bottom2)
    }
}
