use super::ForceGeneratorEntry;
use crate::basics::ParticleEntry;

pub type ForceRegistryEntry = (ForceGeneratorEntry, ParticleEntry);

pub struct ForceRegistry {
    pub entries: Vec<ForceRegistryEntry>,
}

#[allow(dead_code)]
impl ForceRegistry {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn add(&mut self, force_gen: ForceGeneratorEntry, particle: ParticleEntry) {
        self.entries.push((force_gen, particle));
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn update_forces(&mut self, dt: f32) {
        for i in 0..self.entries.len() {
            let (force_gen, particle) = &mut self.entries[i];
            force_gen.update_force(particle, dt);
        }
    }
    pub fn get_particle_mut(&mut self, index: usize) -> Option<&mut ParticleEntry> {
        self.entries.get_mut(index).map(|(_, p)| p)
    }

    pub fn remove_at(&mut self, index: usize) -> Option<ForceRegistryEntry> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }
}
