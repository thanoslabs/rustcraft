pub mod chunk_lifecycle;
//! Voxel blocks, chunk sections, and world storage.

use rustcraft_core::{CHUNK_VOLUME, LocalPos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BlockState(pub u16);

impl BlockState {
    pub const AIR: Self = Self(0);
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
    blocks: Box<[BlockState; CHUNK_VOLUME]>,
    non_air_count: u16,
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkSection {
    pub fn new() -> Self {
        Self {
            blocks: Box::new([BlockState::AIR; CHUNK_VOLUME]),
            non_air_count: 0,
        }
    }
    pub fn get(&self, pos: LocalPos) -> BlockState {
        self.blocks[pos.index()]
    }
    pub fn set(&mut self, pos: LocalPos, state: BlockState) -> BlockState {
        let previous = std::mem::replace(&mut self.blocks[pos.index()], state);
        if previous == BlockState::AIR && state != BlockState::AIR {
            self.non_air_count += 1;
        }
        if previous != BlockState::AIR && state == BlockState::AIR {
            self.non_air_count -= 1;
        }
        previous
    }
    pub fn non_air_count(&self) -> u16 {
        self.non_air_count
    }
    pub fn is_empty(&self) -> bool {
        self.non_air_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn setting_and_replacing_blocks_updates_occupancy() {
        let mut section = ChunkSection::new();
        let pos = LocalPos::new(1, 2, 3).unwrap();
        assert!(section.is_empty());
        assert_eq!(section.set(pos, BlockState(1)), BlockState::AIR);
        assert_eq!(section.set(pos, BlockState(2)), BlockState(1));
        assert_eq!(section.non_air_count(), 1);
        assert_eq!(section.set(pos, BlockState::AIR), BlockState(2));
        assert!(section.is_empty());
    }
}

pub mod terrain;

pub mod meshing;

pub mod streaming;
