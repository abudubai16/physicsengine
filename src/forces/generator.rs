use crate::basics::ParticleEntry;

#[allow(dead_code)]
pub type ForceGeneratorEntry = Box<dyn ParticleForceGenerator>;

pub trait ParticleForceGenerator {
    fn update_force(&self, particle: &mut ParticleEntry, dt: f32);
}
