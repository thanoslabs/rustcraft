//! CPU-side visible-face meshing of a single chunk section.
//! Optional adjacent sections suppress internal faces across section boundaries.
use crate::{BlockState, ChunkSection};
use rustcraft_core::LocalPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Left,
    Right,
    Bottom,
    Top,
    Back,
    Front,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisibleFace {
    pub position: LocalPos,
    pub face: Face,
    pub block: BlockState,
}

pub fn collect_visible_faces(section: &ChunkSection) -> Vec<VisibleFace> {
    collect_visible_faces_with_neighbors(section, [None; 6])
}

/// Neighbors are ordered left, right, bottom, top, back, front.
/// Missing neighbors expose boundary faces so streaming never creates holes.
pub fn collect_visible_faces_with_neighbors(
    section: &ChunkSection,
    neighbors: [Option<&ChunkSection>; 6],
) -> Vec<VisibleFace> {
    let mut faces = Vec::new();
    let directions = [
        (-1_i16, 0, 0, Face::Left),
        (1, 0, 0, Face::Right),
        (0, -1, 0, Face::Bottom),
        (0, 1, 0, Face::Top),
        (0, 0, -1, Face::Back),
        (0, 0, 1, Face::Front),
    ];
    for y in 0..16_u8 {
        for z in 0..16_u8 {
            for x in 0..16_u8 {
                let position = LocalPos::new(x, y, z).expect("in bounds");
                let block = section.get(position);
                if block == BlockState::AIR {
                    continue;
                }
                for (direction_index, &(dx, dy, dz, face)) in directions.iter().enumerate() {
                    let (nx, ny, nz) = (i16::from(x) + dx, i16::from(y) + dy, i16::from(z) + dz);
                    let exposed = if (0..16).contains(&nx)
                        && (0..16).contains(&ny)
                        && (0..16).contains(&nz)
                    {
                        let neighbor =
                            LocalPos::new(nx as u8, ny as u8, nz as u8).expect("in bounds");
                        section.get(neighbor) == BlockState::AIR
                    } else {
                        neighbors[direction_index].is_none_or(|neighbor| {
                            let wrapped = LocalPos::new(
                                nx.rem_euclid(16) as u8,
                                ny.rem_euclid(16) as u8,
                                nz.rem_euclid(16) as u8,
                            )
                            .expect("wrapped coordinates are in bounds");
                            neighbor.get(wrapped) == BlockState::AIR
                        })
                    };
                    if exposed {
                        faces.push(VisibleFace {
                            position,
                            face,
                            block,
                        });
                    }
                }
            }
        }
    }
    faces
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_chunk_has_no_faces() {
        assert!(collect_visible_faces(&ChunkSection::new()).is_empty());
    }
    #[test]
    fn adjacent_blocks_hide_internal_faces() {
        let mut chunk = ChunkSection::new();
        chunk.set(LocalPos::new(1, 1, 1).unwrap(), BlockState(1));
        chunk.set(LocalPos::new(2, 1, 1).unwrap(), BlockState(1));
        assert_eq!(collect_visible_faces(&chunk).len(), 10);
    }
    #[test]
    fn adjacent_sections_hide_shared_boundary_faces() {
        let mut center = ChunkSection::new();
        let mut right = ChunkSection::new();
        center.set(LocalPos::new(15, 4, 6).unwrap(), BlockState(1));
        right.set(LocalPos::new(0, 4, 6).unwrap(), BlockState(1));
        assert_eq!(collect_visible_faces(&center).len(), 6);
        assert_eq!(
            collect_visible_faces_with_neighbors(
                &center,
                [None, Some(&right), None, None, None, None],
            )
            .len(),
            5
        );
        assert_eq!(
            collect_visible_faces_with_neighbors(
                &right,
                [Some(&center), None, None, None, None, None],
            )
            .len(),
            5
        );
    }

    #[test]
    fn solid_chunk_has_only_boundary_faces() {
        let mut chunk = ChunkSection::new();
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    chunk.set(LocalPos::new(x, y, z).unwrap(), BlockState(1));
                }
            }
        }
        assert_eq!(collect_visible_faces(&chunk).len(), 6 * 16 * 16);
    }
}
