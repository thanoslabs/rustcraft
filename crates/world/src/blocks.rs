//! Classic survival block identifiers and gameplay properties.
//! IDs 1-3 are kept stable for existing generated terrain.
use crate::BlockState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockProperties {
    pub name: &'static str,
    pub solid: bool,
    pub opaque: bool,
    pub hardness: u8,
}

pub const AIR: BlockState = BlockState(0);
pub const GRASS: BlockState = BlockState(1);
pub const DIRT: BlockState = BlockState(2);
pub const STONE: BlockState = BlockState(3);
pub const COBBLESTONE: BlockState = BlockState(4);
pub const PLANKS: BlockState = BlockState(5);
pub const BEDROCK: BlockState = BlockState(6);
pub const WATER: BlockState = BlockState(7);
pub const SAND: BlockState = BlockState(8);
pub const GRAVEL: BlockState = BlockState(9);
pub const LOG: BlockState = BlockState(10);
pub const LEAVES: BlockState = BlockState(11);
pub const GLASS: BlockState = BlockState(12);
pub const TORCH: BlockState = BlockState(13);
pub const CRAFTING_TABLE: BlockState = BlockState(14);
pub const FURNACE: BlockState = BlockState(15);
pub const CHEST: BlockState = BlockState(16);

pub fn properties(state: BlockState) -> Option<BlockProperties> {
    let (name, solid, opaque, hardness) = match state.0 {
        0 => ("air", false, false, 0),
        1 => ("grass", true, true, 1),
        2 => ("dirt", true, true, 1),
        3 => ("stone", true, true, 3),
        4 => ("cobblestone", true, true, 3),
        5 => ("planks", true, true, 2),
        6 => ("bedrock", true, true, u8::MAX),
        7 => ("water", false, false, 0),
        8 => ("sand", true, true, 1),
        9 => ("gravel", true, true, 1),
        10 => ("log", true, true, 2),
        11 => ("leaves", true, false, 1),
        12 => ("glass", true, false, 1),
        13 => ("torch", false, false, 0),
        14 => ("crafting_table", true, true, 2),
        15 => ("furnace", true, true, 3),
        16 => ("chest", true, true, 2),
        _ => return None,
    };
    Some(BlockProperties {
        name,
        solid,
        opaque,
        hardness,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_terrain_ids_remain_stable() {
        assert_eq!(GRASS.0, 1);
        assert_eq!(DIRT.0, 2);
        assert_eq!(STONE.0, 3);
    }

    #[test]
    fn fluids_and_air_do_not_block_player_movement() {
        assert!(!properties(AIR).unwrap().solid);
        assert!(!properties(WATER).unwrap().solid);
        assert!(properties(STONE).unwrap().solid);
    }

    #[test]
    fn unknown_ids_are_not_silently_treated_as_air() {
        assert_eq!(properties(BlockState(u16::MAX)), None);
    }
}
