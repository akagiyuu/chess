pub mod bundle;
pub mod generator;
pub mod mesh;

use bevy::prelude::*;
use generator::{PieceGenerator, Point};
use bundle::*;
use strum_macros::EnumIter;

fn bundle(mesh: Handle<Mesh>, material: Handle<StandardMaterial>, position: Vec3) -> PbrBundle {
    PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(position).with_scale(Vec3::new(0.2, 0.2, 0.2)),
        ..Default::default()
    }
}

macro_rules! bundle {
    ($mesh:expr, $material:expr, $position:expr) => {
        (
            PbrBundle {
                mesh: $mesh,
                material: $material,
                transform: Transform::from_translation($position).with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            },

        )
    };
}

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_black = materials.add(Color::rgb(0., 0.2, 0.2).into());
    let material_white = materials.add(Color::rgb(1., 0.8, 0.8).into());

    let queen_mesh: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh3/Primitive0");
    let rook_mesh: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh4/Primitive0");
    let bishop_mesh: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh0/Primitive0");
    let pawn_mesh: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh2/Primitive0");
    let king_mesh: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh5/Primitive0");
    let knight_mesh: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh1/Primitive0");

    commands.spawn_batch([bundle!(
        rook_mesh.clone(),
        material_black.clone(),
        Vec3::new(0., 0., 0.)
    )])
}
