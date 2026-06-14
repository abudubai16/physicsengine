use super::vector::Vector;
use macroquad::prelude::Color;

#[allow(dead_code)]
pub trait Shape {
    fn position(&self) -> Vector;
    fn velocity(&self) -> Vector;
    fn rotation(&self) -> Vector;
    fn ang_vel(&self) -> Vector;

    fn color(&self) -> Color;

    fn new() -> Self
    where
        Self: Sized;

    fn update_position(&mut self);
    fn update_rotation(&mut self);
}
