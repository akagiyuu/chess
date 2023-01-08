use super::mesh::PieceMesh;
use super::PieceKind;
use super::PieceSide;
use bevy::{prelude::*, utils::HashMap};
use strum::IntoEnumIterator;

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct PieceGenerator {
    materials: HashMap<PieceSide, Handle<StandardMaterial>>,
    meshes: HashMap<PieceKind, PieceMesh>,
}

impl PieceGenerator {
    pub fn new(
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mut meshes = HashMap::new();
        for kind in PieceKind::iter() {
            meshes.insert(kind, PieceMesh::new(asset_server, kind));
        }

        Self {
            materials: HashMap::from([
                (PieceSide::Black, materials.add(Color::rgb(0., 0.2, 0.2).into())),
                (PieceSide::White, materials.add(Color::rgb(1., 0.8, 0.8).into())),
            ]),
            meshes,
        }
    }
    pub fn spawn(&self, commands: &mut Commands, kind: PieceKind, side: PieceSide, position: Point) {
        let mesh = &self.meshes[&kind];
        let material = &self.materials[&side];
        let position = Vec3::new(position.x, 0., position.y);

        commands
            // Spawn parent entity
            .spawn(PbrBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            // Add children to the parent
            .with_children(|parent| {
                mesh.cached.iter().for_each(|mesh_handle| {
                    parent.spawn(PbrBundle {
                        mesh: mesh_handle.clone(),
                        material: material.clone(),
                        transform: Transform::from_translation(mesh.align)
                            .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                        ..Default::default()
                    });
                })
            });
    }
}
