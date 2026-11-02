//! Shared mining and placement rules for generated terrain with player overrides.
use crate::{BlockState, edits::{BlockPosition, WorldEdits}, terrain::generate_section};
use rustcraft_core::ChunkPos;

pub struct EditableWorld {
    pub seed: u64,
    pub edits: WorldEdits,
}

impl EditableWorld {
    pub fn new(seed: u64) -> Self {
        Self { seed, edits: WorldEdits::default() }
    }

    pub fn block(&self, position: BlockPosition) -> BlockState {
        if let Some(block) = self.edits.get(position) {
            return block;
        }
        let (chunk, local) = position.split();
        generate_section(self.seed, chunk).get(local)
    }

    /// Mine a non-air block and return its former block state.
    pub fn mine(&mut self, position: BlockPosition) -> Option<BlockState> {
        let previous = self.block(position);
        if previous == BlockState::AIR {
            return None;
        }
        self.edits.set(position, BlockState::AIR);
        Some(previous)
    }

    /// Place only into empty space; returns false if the cell is occupied.
    pub fn place(&mut self, position: BlockPosition, block: BlockState) -> bool {
        if block == BlockState::AIR || self.block(position) != BlockState::AIR {
            return false;
        }
        self.edits.set(position, block);
        true
    }

    pub fn section(&self, chunk: ChunkPos) -> crate::ChunkSection {
        let mut section = generate_section(self.seed, chunk);
        self.edits.apply_to_section(chunk, &mut section);
        section
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mining_and_placement_survive_regeneration() {
        let mut world = EditableWorld::new(42);
        let pos = BlockPosition { x: -1, y: 0, z: -1 };
        let original = world.block(pos);
        assert_ne!(original, BlockState::AIR);
        assert_eq!(world.mine(pos), Some(original));
        assert_eq!(world.block(pos), BlockState::AIR);
        assert!(world.place(pos, BlockState(5)));
        assert!(!world.place(pos, BlockState(3)));
        let (chunk, local) = pos.split();
        assert_eq!(world.section(chunk).get(local), BlockState(5));
    }

    #[test]
    fn cannot_mine_air_or_place_air() {
        let mut world = EditableWorld::new(42);
        let pos = BlockPosition { x: 0, y: 100, z: 0 };
        assert_eq!(world.mine(pos), None);
        assert!(!world.place(pos, BlockState::AIR));
    }
}
