use crate::board::resource::BoardMatrix;
use bevy::prelude::*;

use super::bundle::PieceBundle;
use super::component::{PieceKind, PieceSide};
use super::resource::PieceMaterial;
use super::resource::PieceMesh;

pub fn spawn_pieces(mut commands: Commands, materials: Res<PieceMaterial>, meshes: Res<PieceMesh>) {
    let bundle = |kind: PieceKind, side: PieceSide, position: Vec3| -> (PbrBundle, PieceBundle) {
        (
            PbrBundle {
                mesh: meshes.get(&kind),
                material: materials.get(&side),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            },
            PieceBundle {
                kind,
                side,
                ..Default::default()
            },
        )
    };

    commands.spawn_batch([
        bundle(PieceKind::Rook, PieceSide::Black, Vec3::new(0., 0., 0.)),
        bundle(PieceKind::Knight, PieceSide::Black, Vec3::new(0., 0., 1.)),
        bundle(PieceKind::Bishop, PieceSide::Black, Vec3::new(0., 0., 2.)),
        bundle(PieceKind::King, PieceSide::Black, Vec3::new(0., 0., 3.)),
        bundle(PieceKind::Queen, PieceSide::Black, Vec3::new(0., 0., 4.)),
        bundle(PieceKind::Bishop, PieceSide::Black, Vec3::new(0., 0., 5.)),
        bundle(PieceKind::Knight, PieceSide::Black, Vec3::new(0., 0., 6.)),
        bundle(PieceKind::Rook, PieceSide::Black, Vec3::new(0., 0., 7.)),
    ]);
    for i in 0..8 {
        commands.spawn(bundle(
            PieceKind::Pawn,
            PieceSide::Black,
            Vec3::new(1., 0., i as f32),
        ));
    }

    commands.spawn_batch([
        bundle(PieceKind::Rook, PieceSide::White, Vec3::new(7., 0., 0.)),
        bundle(PieceKind::Knight, PieceSide::White, Vec3::new(7., 0., 1.)),
        bundle(PieceKind::Bishop, PieceSide::White, Vec3::new(7., 0., 2.)),
        bundle(PieceKind::King, PieceSide::White, Vec3::new(7., 0., 3.)),
        bundle(PieceKind::Queen, PieceSide::White, Vec3::new(7., 0., 4.)),
        bundle(PieceKind::Bishop, PieceSide::White, Vec3::new(7., 0., 5.)),
        bundle(PieceKind::Knight, PieceSide::White, Vec3::new(7., 0., 6.)),
        bundle(PieceKind::Rook, PieceSide::White, Vec3::new(7., 0., 7.)),
    ]);
    for i in 0..8 {
        commands.spawn(bundle(
            PieceKind::Pawn,
            PieceSide::White,
            Vec3::new(6., 0., i as f32),
        ));
    }
}

pub fn assign_board_matrix(mut board_matrix: ResMut<BoardMatrix>) {
    let test = &mut board_matrix.0;
    for i in [0, 1, 6, 7] {
        for j in 0..8 {
            test[i][j] = true;
        }
    }
    println!("{:?}", board_matrix);
}
