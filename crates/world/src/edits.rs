//! Sparse player block edits layered over deterministic generated terrain.
//! This is the in-memory source of truth for edits; disk persistence follows separately.
use rustcraft_core::{ChunkPos, LocalPos, split_coordinate};
use std::collections::HashMap;

use crate::BlockState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPosition {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl BlockPosition {
    pub fn split(self) -> (ChunkPos, LocalPos) {
        let (cx, lx) = split_coordinate(self.x);
        let (cy, ly) = split_coordinate(self.y);
        let (cz, lz) = split_coordinate(self.z);
        (
            ChunkPos { x: cx, y: cy, z: cz },
            LocalPos { x: lx, y: ly, z: lz },
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorldEdits {
    changes: HashMap<ChunkPos, HashMap<usize, BlockState>>,
}

impl WorldEdits {
    /// Store an override, including air for mined blocks.
    pub fn set(&mut self, position: BlockPosition, block: BlockState) -> Option<BlockState> {
        let (chunk, local) = position.split();
        self.changes
            .entry(chunk)
            .or_default()
            .insert(local.index(), block)
    }

    pub fn get(&self, position: BlockPosition) -> Option<BlockState> {
        let (chunk, local) = position.split();
        self.changes.get(&chunk)?.get(&local.index()).copied()
    }

    pub fn edits_for_chunk(
        &self,
        chunk: ChunkPos,
    ) -> impl Iterator<Item = (usize, BlockState)> + '_ {
        self.changes
            .get(&chunk)
            .into_iter()
            .flat_map(|edits| edits.iter().map(|(&index, &block)| (index, block)))
    }

    pub fn edited_chunk_count(&self) -> usize {
        self.changes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_coordinates_use_euclidean_chunks() {
        let (chunk, local) = BlockPosition {
            x: -1,
            y: -17,
            z: 16,
        }
        .split();
        assert_eq!(chunk, ChunkPos { x: -1, y: -2, z: 1 });
        assert_eq!(local, LocalPos { x: 15, y: 15, z: 0 });
    }

    #[test]
    fn mined_air_is_distinct_from_no_override() {
        let mut edits = WorldEdits::default();
        let pos = BlockPosition { x: 0, y: 64, z: 0 };
        assert_eq!(edits.get(pos), None);
        assert_eq!(edits.set(pos, BlockState(0)), None);
        assert_eq!(edits.get(pos), Some(BlockState(0)));
        assert_eq!(edits.edited_chunk_count(), 1);
    }

    #[test]
    fn edits_are_grouped_by_chunk() {
        let mut edits = WorldEdits::default();
        edits.set(BlockPosition { x: 0, y: 0, z: 0 }, BlockState(3));
        edits.set(BlockPosition { x: 16, y: 0, z: 0 }, BlockState(2));
        assert_eq!(edits.edited_chunk_count(), 2);
        assert_eq!(edits.edits_for_chunk(ChunkPos { x: 0, y: 0, z: 0 }).count(), 1);
    }
}
