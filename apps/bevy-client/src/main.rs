//! Bevy terrain renderer backed by Rustcraft's existing chunk generator and face mesher.
use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology,
};
use rustcraft_core::ChunkPos;
use rustcraft_world::{
    ChunkSection,
    meshing::{Face, collect_visible_faces_with_neighbors},
    terrain::generate_section,
};
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rustcraft (Bevy)".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 28.0, 20.0).looking_at(Vec3::new(0.0, 12.0, -20.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.5, 0.0)),
    ));
    let mut sections = HashMap::new();
    for z in -2..=2 {
        for x in -2..=2 {
            for y in 0..=1 {
                let pos = ChunkPos { x, y, z };
                sections.insert(pos, generate_section(42, pos));
            }
        }
    }
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        ..default()
    });
    for (&pos, section) in &sections {
        let neighbors = [
            ChunkPos {
                x: pos.x - 1,
                ..pos
            },
            ChunkPos {
                x: pos.x + 1,
                ..pos
            },
            ChunkPos {
                y: pos.y - 1,
                ..pos
            },
            ChunkPos {
                y: pos.y + 1,
                ..pos
            },
            ChunkPos {
                z: pos.z - 1,
                ..pos
            },
            ChunkPos {
                z: pos.z + 1,
                ..pos
            },
        ];
        if let Some(mesh) = section_mesh(section, neighbors.map(|p| sections.get(&p))) {
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(
                    pos.x as f32 * 16.0,
                    pos.y as f32 * 16.0,
                    pos.z as f32 * 16.0,
                ),
            ));
        }
    }
}

fn section_mesh(section: &ChunkSection, neighbors: [Option<&ChunkSection>; 6]) -> Option<Mesh> {
    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut colors = Vec::<[f32; 4]>::new();
    let mut indices = Vec::<u32>::new();
    for visible in collect_visible_faces_with_neighbors(section, neighbors) {
        let x = f32::from(visible.position.x);
        let y = f32::from(visible.position.y);
        let z = f32::from(visible.position.z);
        let (corners, normal, shade) = match visible.face {
            Face::Left => (
                [[0., 0., 1.], [0., 0., 0.], [0., 1., 0.], [0., 1., 1.]],
                [-1., 0., 0.],
                0.75,
            ),
            Face::Right => (
                [[1., 0., 0.], [1., 0., 1.], [1., 1., 1.], [1., 1., 0.]],
                [1., 0., 0.],
                0.75,
            ),
            Face::Bottom => (
                [[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]],
                [0., -1., 0.],
                0.55,
            ),
            Face::Top => (
                [[0., 1., 1.], [1., 1., 1.], [1., 1., 0.], [0., 1., 0.]],
                [0., 1., 0.],
                1.0,
            ),
            Face::Back => (
                [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [0., 1., 0.]],
                [0., 0., -1.],
                0.85,
            ),
            Face::Front => (
                [[1., 0., 1.], [0., 0., 1.], [0., 1., 1.], [1., 1., 1.]],
                [0., 0., 1.],
                0.85,
            ),
        };
        let base = match visible.block.0 {
            1 => [0.32, 0.65, 0.23],
            2 => [0.48, 0.32, 0.19],
            _ => [0.48, 0.50, 0.53],
        };
        let start = positions.len() as u32;
        for corner in corners {
            positions.push([x + corner[0], y + corner[1], z + corner[2]]);
            normals.push(normal);
            colors.push([base[0] * shade, base[1] * shade, base[2] * shade, 1.0]);
        }
        indices.extend_from_slice(&[start, start + 1, start + 2, start, start + 2, start + 3]);
    }
    if indices.is_empty() {
        return None;
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    Some(mesh)
}

/// Spectator movement until the player physics controller is integrated.
fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    for mut transform in &mut cameras {
        let mut movement = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            movement.z -= 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            movement.z += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }
        if keys.pressed(KeyCode::Space) {
            movement.y += 1.0;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            movement.y -= 1.0;
        }
        if movement != Vec3::ZERO {
            transform.translation += movement.normalize() * 12.0 * time.delta_secs();
        }
    }
}
