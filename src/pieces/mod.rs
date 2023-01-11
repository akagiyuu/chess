pub mod bundle;
pub mod components;
pub mod resources;

use crate::components::{MeshColor, Position};
use bevy::prelude::*;
use bundle::PieceBundle;

use self::components::{Piece, PieceKind};
use self::resources::PieceMaterial;
use self::resources::PieceMesh;

fn init(mut commands: Commands, materials: Res<PieceMaterial>, meshes: Res<PieceMesh>) {
    let bundle =
        |kind: PieceKind, color: MeshColor, position: Vec3| -> (PbrBundle, PieceBundle) {
            (
                PbrBundle {
                    mesh: meshes.get(&kind),
                    material: materials.get(&color),
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                    ..Default::default()
                },
                PieceBundle {
                    kind,
                    color,
                    ..Default::default()
                },
            )
        };

    commands.spawn_batch([
        bundle(PieceKind::Rook, MeshColor::Black, Vec3::new(0., 0., 0.)),
        bundle(
            PieceKind::Knight,
            MeshColor::Black,
            Vec3::new(0., 0., 1.),
        ),
        bundle(
            PieceKind::Bishop,
            MeshColor::Black,
            Vec3::new(0., 0., 2.),
        ),
        bundle(PieceKind::King, MeshColor::Black, Vec3::new(0., 0., 3.)),
        bundle(
            PieceKind::Queen,
            MeshColor::Black,
            Vec3::new(0., 0., 4.),
        ),
        bundle(
            PieceKind::Bishop,
            MeshColor::Black,
            Vec3::new(0., 0., 5.),
        ),
        bundle(
            PieceKind::Knight,
            MeshColor::Black,
            Vec3::new(0., 0., 6.),
        ),
        bundle(PieceKind::Rook, MeshColor::Black, Vec3::new(0., 0., 7.)),
    ]);
    for i in 0..8 {
        commands.spawn(bundle(
            PieceKind::Pawn,
            MeshColor::Black,
            Vec3::new(1.,0.,i as f32),
        ));
    }

    commands.spawn_batch([
        bundle(PieceKind::Rook, MeshColor::White, Vec3::new(7., 0., 0.)),
        bundle(
            PieceKind::Knight,
            MeshColor::White,
            Vec3::new(7., 0., 1.),
        ),
        bundle(
            PieceKind::Bishop,
            MeshColor::White,
            Vec3::new(7., 0., 2.),
        ),
        bundle(PieceKind::King, MeshColor::White, Vec3::new(7., 0., 3.)),
        bundle(
            PieceKind::Queen,
            MeshColor::White,
            Vec3::new(7., 0., 4.),
        ),
        bundle(
            PieceKind::Bishop,
            MeshColor::White,
            Vec3::new(7., 0., 5.),
        ),
        bundle(
            PieceKind::Knight,
            MeshColor::White,
            Vec3::new(7., 0., 6.),
        ),
        bundle(PieceKind::Rook, MeshColor::White, Vec3::new(7., 0., 7.)),
    ]);
    for i in 0..8 {
        commands.spawn(bundle(
            PieceKind::Pawn,
            MeshColor::White,
            Vec3::new(6.,0.,i as f32),
        ));
    }
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PieceMaterial>()
            .init_resource::<PieceMesh>()
            .add_startup_system(init);
    }
}
