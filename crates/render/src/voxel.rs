//! Convert exposed voxel faces into a single static GPU-ready triangle list.
use rustcraft_core::ChunkPos;
use rustcraft_world::{
    ChunkSection,
    meshing::{Face, collect_visible_faces},
};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub fn append_section_mesh(vertices: &mut Vec<Vertex>, section: &ChunkSection, chunk: ChunkPos) {
    let origin = [
        chunk.x as f32 * 16.0,
        chunk.y as f32 * 16.0,
        chunk.z as f32 * 16.0,
    ];
    for visible in collect_visible_faces(section) {
        let p = [
            origin[0] + f32::from(visible.position.x),
            origin[1] + f32::from(visible.position.y),
            origin[2] + f32::from(visible.position.z),
        ];
        let (corners, shade): ([[f32; 3]; 4], f32) = match visible.face {
            Face::Left => (
                [[0., 0., 1.], [0., 0., 0.], [0., 1., 0.], [0., 1., 1.]],
                0.75,
            ),
            Face::Right => (
                [[1., 0., 0.], [1., 0., 1.], [1., 1., 1.], [1., 1., 0.]],
                0.75,
            ),
            Face::Bottom => (
                [[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]],
                0.55,
            ),
            Face::Top => (
                [[0., 1., 1.], [1., 1., 1.], [1., 1., 0.], [0., 1., 0.]],
                1.0,
            ),
            Face::Back => (
                [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [0., 1., 0.]],
                0.85,
            ),
            Face::Front => (
                [[1., 0., 1.], [0., 0., 1.], [0., 1., 1.], [1., 1., 1.]],
                0.85,
            ),
        };
        let base: [f32; 3] = match visible.block.0 {
            1 => [0.32, 0.65, 0.23],
            2 => [0.48, 0.32, 0.19],
            _ => [0.48, 0.50, 0.53],
        };
        // Subtle deterministic per-block tint makes stacked cube boundaries
        // visible without introducing texture assets or nondeterminism.
        let hash = (p[0] as i64).wrapping_mul(73_856_093)
            ^ (p[1] as i64).wrapping_mul(19_349_663)
            ^ (p[2] as i64).wrapping_mul(83_492_791);
        let variation = 0.91 + (hash.rem_euclid(11) as f32) * 0.012;
        let color = base.map(|c| c * shade * variation);
        for i in [0, 1, 2, 0, 2, 3] {
            let offset = corners[i];
            vertices.push(Vertex {
                position: [p[0] + offset[0], p[1] + offset[1], p[2] + offset[2]],
                color,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustcraft_core::LocalPos;
    use rustcraft_world::BlockState;

    #[test]
    fn single_block_produces_six_quads() {
        let mut section = ChunkSection::new();
        section.set(LocalPos::new(0, 0, 0).unwrap(), BlockState(1));
        let mut vertices = Vec::new();
        append_section_mesh(&mut vertices, &section, ChunkPos { x: 0, y: 0, z: 0 });
        assert_eq!(vertices.len(), 36);
    }
}
