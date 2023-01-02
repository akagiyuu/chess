pub mod asset;
pub mod generator;
pub mod mesh;
pub mod kind;

use bevy::prelude::*;
use generator::{Generator, Kind, Point, Side};

pub fn init(
    mut commands: Commands,
    assset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pieces_generator = Generator::new(&assset_server, &mut materials);

    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    pieces_generator.spawn(
        &mut commands,
        Kind::Rook(Side::White, Point { x: 0., y: 0. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Knight(Side::White, Point { x: 0., y: 1. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Bishop(Side::White, Point { x: 0., y: 2. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Queen(Side::White, Point { x: 0., y: 3. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::King(Side::White, Point { x: 0., y: 4. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Bishop(Side::White, Point { x: 0., y: 5. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Knight(Side::White, Point { x: 0., y: 6. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Rook(Side::White, Point { x: 0., y: 7. }),
    );

    for i in 0..8 {
        pieces_generator.spawn(
            &mut commands,
            Kind::Pawn(Side::White, Point { x: 1., y: i as f32 }),
        );
    }

    pieces_generator.spawn(
        &mut commands,
        Kind::Rook(Side::Black, Point { x: 7., y: 0. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Knight(Side::Black, Point { x: 7., y: 1. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Bishop(Side::Black, Point { x: 7., y: 2. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Queen(Side::Black, Point { x: 7., y: 3. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::King(Side::Black, Point { x: 7., y: 4. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Bishop(Side::Black, Point { x: 7., y: 5. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Knight(Side::Black, Point { x: 7., y: 6. }),
    );
    pieces_generator.spawn(
        &mut commands,
        Kind::Rook(Side::Black, Point { x: 7., y: 7. }),
    );

    for i in 0..8 {
        pieces_generator.spawn(
            &mut commands,
            Kind::Pawn(Side::Black, Point { x: 6., y: i as f32 }),
        );
    }
}
