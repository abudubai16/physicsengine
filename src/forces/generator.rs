use crate::basics::ParticleStore;

#[allow(dead_code)]
pub type ForceGeneratorEntry = Box<dyn ParticleForceGenerator>;

pub trait ParticleForceGenerator {
    fn update_force(&self, particle_store: &mut ParticleStore, particle_index: usize, dt: f32);
    fn draw_force(&self, _: &ParticleStore, _: usize) {
        // Default implementation does nothing
    }
}
