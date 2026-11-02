//! Grid traversal for first-person block targeting.
use crate::edits::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockHit {
    pub block: BlockPosition,
    pub previous: Option<BlockPosition>,
    pub distance: f64,
}

/// Traverse voxel cells along a normalized ray, stopping at the first occupied block.
/// The caller supplies the occupancy lookup so generated terrain and player edits share the path.
pub fn raycast(
    origin: [f64; 3],
    direction: [f64; 3],
    reach: f64,
    mut occupied: impl FnMut(BlockPosition) -> bool,
) -> Option<BlockHit> {
    if !origin.iter().chain(direction.iter()).all(|v| v.is_finite())
        || !reach.is_finite()
        || reach < 0.0
    {
        return None;
    }
    let length = direction.iter().map(|v| v * v).sum::<f64>().sqrt();
    if length <= f64::EPSILON {
        return None;
    }
    let dir = direction.map(|v| v / length);
    let mut cell = origin.map(|v| v.floor() as i64);
    let step = dir.map(|v| {
        if v > 0.0 {
            1_i64
        } else if v < 0.0 {
            -1
        } else {
            0
        }
    });
    let mut next = [f64::INFINITY; 3];
    let mut delta = [f64::INFINITY; 3];
    for axis in 0..3 {
        if step[axis] != 0 {
            let boundary = if step[axis] > 0 {
                cell[axis] as f64 + 1.0
            } else {
                cell[axis] as f64
            };
            next[axis] = ((boundary - origin[axis]) / dir[axis]).max(0.0);
            delta[axis] = 1.0 / dir[axis].abs();
        }
    }
    let mut previous = None;
    let mut distance = 0.0;
    loop {
        let block = BlockPosition {
            x: cell[0],
            y: cell[1],
            z: cell[2],
        };
        if occupied(block) {
            return Some(BlockHit {
                block,
                previous,
                distance,
            });
        }
        let axis = if next[0] <= next[1] && next[0] <= next[2] {
            0
        } else if next[1] <= next[2] {
            1
        } else {
            2
        };
        if next[axis] > reach {
            return None;
        }
        distance = next[axis];
        previous = Some(block);
        cell[axis] += step[axis];
        next[axis] += delta[axis];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hits_block_and_reports_placement_cell() {
        let hit = raycast([0.5, 0.5, 0.5], [1.0, 0.0, 0.0], 5.0, |p| p.x == 3).unwrap();
        assert_eq!(hit.block.x, 3);
        assert_eq!(hit.previous.unwrap().x, 2);
        assert_eq!(hit.distance, 2.5);
    }

    #[test]
    fn handles_negative_coordinates() {
        let hit = raycast([0.5, 0.5, 0.5], [-1.0, 0.0, 0.0], 4.0, |p| p.x == -2).unwrap();
        assert_eq!(hit.block.x, -2);
        assert_eq!(hit.previous.unwrap().x, -1);
    }

    #[test]
    fn reach_and_invalid_direction_are_respected() {
        assert!(raycast([0.5; 3], [1.0, 0.0, 0.0], 1.0, |p| p.x == 3).is_none());
        assert!(raycast([0.5; 3], [0.0; 3], 5.0, |_| true).is_none());
    }
}
