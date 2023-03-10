use bevy::prelude::*;

use super::component::{PieceKind, PieceSide};

#[derive(Resource)]
pub struct PieceMaterial {
    black: Handle<StandardMaterial>,
    white: Handle<StandardMaterial>,
}
impl FromWorld for PieceMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        Self {
            black: materials.add(Color::rgb(0., 0.2, 0.2).into()),
            white: materials.add(Color::rgb(1., 0.8, 0.8).into()),
        }
    }
}
impl PieceMaterial {
    pub fn get(&self, color: &PieceSide) -> Handle<StandardMaterial> {
        match color {
            PieceSide::White => self.black.clone(),
            PieceSide::Black => self.white.clone(),
        }
    }
}

#[derive(Resource)]
pub struct PieceMesh {
    king: Handle<Mesh>,
    queen: Handle<Mesh>,
    pawn: Handle<Mesh>,
    rook: Handle<Mesh>,
    knight: Handle<Mesh>,
    bishop: Handle<Mesh>,
}
impl FromWorld for PieceMesh {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        Self {
            king: asset_server.load("models/pieces.glb#Mesh5/Primitive0"),
            pawn: asset_server.load("models/pieces.glb#Mesh2/Primitive0"),
            queen: asset_server.load("models/pieces.glb#Mesh3/Primitive0"),
            rook: asset_server.load("models/pieces.glb#Mesh4/Primitive0"),
            bishop: asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
            knight: asset_server.load("models/pieces.glb#Mesh1/Primitive0"),
        }
    }
}
impl PieceMesh {
    pub fn get(&self, kind: &PieceKind) -> Handle<Mesh> {
        match kind {
            PieceKind::King => self.king.clone(),
            PieceKind::Queen => self.queen.clone(),
            PieceKind::Rook => self.rook.clone(),
            PieceKind::Knight => self.knight.clone(),
            PieceKind::Bishop => self.bishop.clone(),
            PieceKind::Pawn => self.pawn.clone(),
        }
    }
}
