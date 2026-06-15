use crate::basics::ParticleEntry;

#[allow(dead_code)]
pub type ForceGeneratorEntry = Box<dyn ForceGenerator>;

pub trait ForceGenerator {
    fn update_force(&self, particle: &mut ParticleEntry, dt: f32);
}
