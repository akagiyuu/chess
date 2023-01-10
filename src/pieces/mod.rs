pub mod bundle;
pub mod components;
pub mod resources;

use crate::components::{MeshColor, Position};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bundle::PieceBundle;
use components::PieceKind;
use resources::PieceMaterial;

use self::resources::PieceMesh;

pub fn init(mut commands: Commands, materials: Res<PieceMaterial>, meshes: Res<PieceMesh>) {
    let bundle =
        |kind: PieceKind, color: MeshColor, position: Position| -> (PbrBundle, PieceBundle) {
            (
                PbrBundle {
                    mesh: meshes.get(&kind),
                    material: materials.get(&color),
                    transform: Transform::from_xyz(position.x, 0., position.y)
                        .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                    ..Default::default()
                },
                PieceBundle {
                    kind,
                    color,
                    position,
                },
            )
        };

    commands.spawn_batch([
        bundle(PieceKind::Rook, MeshColor::Black, Position { x: 0., y: 0. }),
        bundle(
            PieceKind::Knight,
            MeshColor::Black,
            Position { x: 0., y: 1. },
        ),
        bundle(
            PieceKind::Bishop,
            MeshColor::Black,
            Position { x: 0., y: 2. },
        ),
        bundle(PieceKind::King, MeshColor::Black, Position { x: 0., y: 3. }),
        bundle(
            PieceKind::Queen,
            MeshColor::Black,
            Position { x: 0., y: 4. },
        ),
        bundle(
            PieceKind::Bishop,
            MeshColor::Black,
            Position { x: 0., y: 5. },
        ),
        bundle(
            PieceKind::Knight,
            MeshColor::Black,
            Position { x: 0., y: 6. },
        ),
        bundle(PieceKind::Rook, MeshColor::Black, Position { x: 0., y: 7. }),
    ]);
    for i in 0..8 {
        commands.spawn(bundle(
            PieceKind::Pawn,
            MeshColor::Black,
            Position { x: 1., y: i as f32 },
        ));
    }

    commands.spawn_batch([
        bundle(PieceKind::Rook, MeshColor::White, Position { x: 7., y: 0. }),
        bundle(
            PieceKind::Knight,
            MeshColor::White,
            Position { x: 7., y: 1. },
        ),
        bundle(
            PieceKind::Bishop,
            MeshColor::White,
            Position { x: 7., y: 2. },
        ),
        bundle(PieceKind::King, MeshColor::White, Position { x: 7., y: 3. }),
        bundle(
            PieceKind::Queen,
            MeshColor::White,
            Position { x: 7., y: 4. },
        ),
        bundle(
            PieceKind::Bishop,
            MeshColor::White,
            Position { x: 7., y: 5. },
        ),
        bundle(
            PieceKind::Knight,
            MeshColor::White,
            Position { x: 7., y: 6. },
        ),
        bundle(PieceKind::Rook, MeshColor::White, Position { x: 7., y: 7. }),
    ]);
    for i in 0..8 {
        commands.spawn(bundle(
            PieceKind::Pawn,
            MeshColor::White,
            Position { x: 6., y: i as f32 },
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
