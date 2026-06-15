use super::vector::Vector;
use macroquad::prelude::Color;

#[allow(dead_code)]
pub type ParticleEntry = Box<dyn Particle>;

///
/// A trait representing a particle in a physics simulation. It defines the necessary properties and methods for implementation of a particle, including position, velocity, rotation, damping, mass and force accumulation.
///
pub trait Particle {
    fn position(&self) -> Vector;
    fn velocity(&self) -> Vector;
    fn rotation(&self) -> Vector;
    fn ang_vel(&self) -> Vector;
    fn damping(&self) -> f32;
    fn inv_mass(&self) -> f32;
    fn inv_inertia(&self) -> f32;
    fn force_accumulator(&self) -> Vector;

    fn clear_accumulator(&mut self);
    fn integrate(&mut self, dt: f32);
    fn add_force(&mut self, force: &Vector);

    fn color(&self) -> Color;
    fn new() -> Self
    where
        Self: Sized;
}
