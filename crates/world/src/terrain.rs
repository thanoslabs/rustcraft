//! Deterministic initial heightmap generator.
//! This is a fast, dependency-free fixture; biomes, caves, and noise follow.
use crate::{BlockState, ChunkSection};
use rustcraft_core::{ChunkPos, LocalPos};

pub const GRASS: BlockState = BlockState(1);
pub const DIRT: BlockState = BlockState(2);
pub const STONE: BlockState = BlockState(3);

fn mix(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

fn cell_height(seed: u64, x: i64, z: i64) -> f64 {
    let h = mix(seed ^ mix(x as u64) ^ mix((z as u64).rotate_left(32)));
    8.0 + (h % 12) as f64
}

fn terrain_height(seed: u64, x: i64, z: i64) -> i64 {
    // Smoothly interpolate coarse height samples so adjacent columns form
    // recognizable one-block steps instead of abrupt eight-block cliffs.
    let cx = x.div_euclid(8);
    let cz = z.div_euclid(8);
    let tx = x.rem_euclid(8) as f64 / 8.0;
    let tz = z.rem_euclid(8) as f64 / 8.0;
    let smooth = |t: f64| t * t * (3.0 - 2.0 * t);
    let (tx, tz) = (smooth(tx), smooth(tz));
    let a = cell_height(seed, cx, cz);
    let b = cell_height(seed, cx + 1, cz);
    let c = cell_height(seed, cx, cz + 1);
    let d = cell_height(seed, cx + 1, cz + 1);
    ((a * (1.0 - tx) + b * tx) * (1.0 - tz) + (c * (1.0 - tx) + d * tx) * tz).round() as i64
}

pub fn generate_section(seed: u64, chunk: ChunkPos) -> ChunkSection {
    let mut section = ChunkSection::new();
    for z in 0..16_u8 {
        for x in 0..16_u8 {
            let world_x = chunk.x.saturating_mul(16).saturating_add(i64::from(x));
            let world_z = chunk.z.saturating_mul(16).saturating_add(i64::from(z));
            let top = terrain_height(seed, world_x, world_z);
            for y in 0..16_u8 {
                let world_y = chunk.y.saturating_mul(16).saturating_add(i64::from(y));
                let block = if world_y > top {
                    BlockState::AIR
                } else if world_y == top {
                    GRASS
                } else if world_y >= top - 3 {
                    DIRT
                } else {
                    STONE
                };
                if block != BlockState::AIR {
                    section.set(LocalPos::new(x, y, z).expect("in bounds"), block);
                }
            }
        }
    }
    section
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_seed_and_position_are_deterministic() {
        let pos = ChunkPos { x: -3, y: 0, z: 7 };
        let a = generate_section(42, pos);
        let b = generate_section(42, pos);
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    let p = LocalPos::new(x, y, z).unwrap();
                    assert_eq!(a.get(p), b.get(p));
                }
            }
        }
    }
    #[test]
    fn sections_below_ground_are_solid() {
        let section = generate_section(42, ChunkPos { x: 0, y: -1, z: 0 });
        assert_eq!(section.non_air_count(), 4096);
    }
}
