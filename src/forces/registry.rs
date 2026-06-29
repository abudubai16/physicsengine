use super::ForceGeneratorEntry;
use crate::basics::ParticleStore;

/// (ForceGenerator, ParticleIndex) to be used within ForceRegistry
pub type ForceRegistryEntry = (ForceGeneratorEntry, usize);

pub struct ForceRegistry {
    entries: Vec<ForceRegistryEntry>,
}

#[allow(dead_code)]
impl ForceRegistry {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, force_gen: ForceGeneratorEntry, particle_index: usize) {
        self.entries.push((force_gen, particle_index));
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn update_forces(&self, particle_store: &mut ParticleStore, dt: f32) {
        for i in 0..self.entries.len() {
            let (force_gen, particle_index) = &self.entries[i];
            force_gen.update_force(particle_store, *particle_index, dt);
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<ForceRegistryEntry> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

    pub fn num_entries(&self) -> usize {
        self.entries.len()
    }

    pub fn get_entry(&self, index: usize) -> &ForceRegistryEntry {
        assert!(index < self.entries.len(), "Index out of bounds");
        self.entries.get(index).unwrap()
    }
}
