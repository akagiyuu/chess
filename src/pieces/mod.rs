pub mod bundle;
pub mod components;

use bevy::{prelude::*, utils::HashMap};
use bundle::*;
use components::*;

fn bundle(mesh: Handle<Mesh>, material: Handle<StandardMaterial>, position: Vec3) -> PbrBundle {
    PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(position).with_scale(Vec3::new(0.2, 0.2, 0.2)),
        ..Default::default()
    }
}

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = HashMap::from([
        (
            PieceSide::Black,
            materials.add(Color::rgb(0., 0.2, 0.2).into()),
        ),
        (
            PieceSide::White,
            materials.add(Color::rgb(1., 0.8, 0.8).into()),
        ),
    ]);

    let mesh: HashMap<PieceKind, Handle<Mesh>> = HashMap::from([
        (
            PieceKind::King,
            asset_server.load("models/pieces.glb#Mesh5/Primitive0"),
        ),
        (
            PieceKind::Pawn,
            asset_server.load("models/pieces.glb#Mesh2/Primitive0"),
        ),
        (
            PieceKind::Queen,
            asset_server.load("models/pieces.glb#Mesh3/Primitive0"),
        ),
        (
            PieceKind::Rook,
            asset_server.load("models/pieces.glb#Mesh4/Primitive0"),
        ),
        (
            PieceKind::Bishop,
            asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
        ),
        (
            PieceKind::Knight,
            asset_server.load("models/pieces.glb#Mesh1/Primitive0"),
        ),
    ]);

    let bundle =
        |kind: PieceKind, side: PieceSide, position: Position| -> (PbrBundle, PieceBundle) {
            (
                PbrBundle {
                    mesh: mesh[&kind].clone(),
                    material: material[&side].clone(),
                    transform: Transform::from_xyz(position.x, 0., position.y)
                        .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                    ..Default::default()
                },
                PieceBundle {
                    kind,
                    side,
                    position,
                },
            )
        };
    commands.spawn_batch([
        bundle(PieceKind::Rook, PieceSide::Black, Position { x: 0., y: 0. }),
        bundle(
            PieceKind::Knight,
            PieceSide::Black,
            Position { x: 0., y: 1. },
        ),
        bundle(
            PieceKind::Bishop,
            PieceSide::Black,
            Position { x: 0., y: 2. },
        ),
        bundle(PieceKind::King, PieceSide::Black, Position { x: 0., y: 3. }),
        bundle(
            PieceKind::Queen,
            PieceSide::Black,
            Position { x: 0., y: 4. },
        ),
        bundle(
            PieceKind::Bishop,
            PieceSide::Black,
            Position { x: 0., y: 5. },
        ),
        bundle(
            PieceKind::Knight,
            PieceSide::Black,
            Position { x: 0., y: 6. },
        ),
        bundle(PieceKind::Rook, PieceSide::Black, Position { x: 0., y: 7. }),
    ]);
    for i in 0..8 {
        commands.spawn(bundle(
            PieceKind::Pawn,
            PieceSide::Black,
            Position { x: 1., y: i as f32 },
        ));
    }

    commands.spawn_batch([
        bundle(PieceKind::Rook, PieceSide::White, Position { x: 7., y: 0. }),
        bundle(
            PieceKind::Knight,
            PieceSide::White,
            Position { x: 7., y: 1. },
        ),
        bundle(
            PieceKind::Bishop,
            PieceSide::White,
            Position { x: 7., y: 2. },
        ),
        bundle(PieceKind::King, PieceSide::White, Position { x: 7., y: 3. }),
        bundle(
            PieceKind::Queen,
            PieceSide::White,
            Position { x: 7., y: 4. },
        ),
        bundle(
            PieceKind::Bishop,
            PieceSide::White,
            Position { x: 7., y: 5. },
        ),
        bundle(
            PieceKind::Knight,
            PieceSide::White,
            Position { x: 7., y: 6. },
        ),
        bundle(PieceKind::Rook, PieceSide::White, Position { x: 7., y: 7. }),
    ]);
    for i in 0..8 {
        commands.spawn(bundle(
            PieceKind::Pawn,
            PieceSide::White,
            Position { x: 6., y: i as f32 },
        ));
    }
}
