use super::mesh::PieceMesh;
use bevy::{prelude::*, utils::HashMap};

pub struct Point {
    pub x: f32,
    pub y: f32,
}
pub enum Side {
    Black,
    White,
}

pub struct Generator {
    black: Handle<StandardMaterial>,
    white: Handle<StandardMaterial>,
    meshes: HashMap<super::kind::Kind, PieceMesh>,
}

impl Generator {
    pub fn new(
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mut cached = HashMap::new();
        for &kind in super::kind::Kind::iter() {
            cached.insert(kind, PieceMesh::new(asset_server, kind));
        }

        Self {
            black: materials.add(Color::rgb(0., 0.2, 0.2).into()),
            white: materials.add(Color::rgb(1., 0.8, 0.8).into()),
            meshes: cached,
        }
    }
    pub fn spawn(&self, commands: &mut Commands, kind: super::kind::Kind, side: Side, position: Point) {
        let mesh = &self.meshes[&kind];
        let material = match side {
            Side::Black => &self.black,
            Side::White => &self.white,
        };
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
