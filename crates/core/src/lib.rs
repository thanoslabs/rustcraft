//! Shared engine types and coordinate primitives.

pub const CHUNK_EDGE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_EDGE * CHUNK_EDGE * CHUNK_EDGE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl LocalPos {
    pub fn new(x: u8, y: u8, z: u8) -> Option<Self> {
        (x < 16 && y < 16 && z < 16).then_some(Self { x, y, z })
    }
    pub fn index(self) -> usize {
        usize::from(self.y) * 256 + usize::from(self.z) * 16 + usize::from(self.x)
    }
}

pub fn split_coordinate(block: i64) -> (i64, u8) {
    (block.div_euclid(16), block.rem_euclid(16) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_coordinates_are_euclidean() {
        assert_eq!(split_coordinate(-1), (-1, 15));
        assert_eq!(split_coordinate(-16), (-1, 0));
        assert_eq!(split_coordinate(-17), (-2, 15));
    }
    #[test]
    fn local_indices_are_unique() {
        let mut seen = [false; CHUNK_VOLUME];
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    let index = LocalPos::new(x, y, z).unwrap().index();
                    assert!(!seen[index]);
                    seen[index] = true;
                }
            }
        }
        assert!(seen.into_iter().all(|value| value));
    }
}
