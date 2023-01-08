use super::PieceKind;
use bevy::prelude::*;

#[derive(Debug)]
pub struct PieceMesh {
    pub cached: Vec<Handle<bevy::prelude::Mesh>>,
    pub align: Vec3,
    pub kind: PieceKind,
}

impl PieceMesh {
    fn new_queen(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh3/Primitive0"),
            ],
            align: Vec3::new(0.,0.,0.),
            kind: PieceKind::Queen,
        }
    }
    fn new_rook(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh4/Primitive0"),
            ],
            align: Vec3::new(0.,0.,0.),
            kind: PieceKind::Rook,
        }
    }
    fn new_bishop(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
            ],
            align: Vec3::new(0.,0.,0.),
            kind: PieceKind::Bishop,
        }
    }
    fn new_pawn(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh2/Primitive0"),
            ],
            align: Vec3::new(0.,0.,0.),
            kind: PieceKind::Pawn,
        }
    }
    fn new_king(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh5/Primitive0"),
            ],
            align: Vec3::new(0.,0.,0.),
            kind: PieceKind::King,
        }
    }
    fn new_knight(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh1/Primitive0"),
            ],
            align: Vec3::new(0.,0.,0.),
            kind: PieceKind::Knight,
        }
    }
    pub fn new(asset_server: &Res<AssetServer>, kind: PieceKind) -> Self {
        match kind {
            PieceKind::King => Self::new_king(asset_server),
            PieceKind::Queen => Self::new_queen(asset_server),
            PieceKind::Rook => Self::new_rook(asset_server),
            PieceKind::Knight => Self::new_knight(asset_server),
            PieceKind::Bishop => Self::new_bishop(asset_server),
            PieceKind::Pawn => Self::new_pawn(asset_server)
        }
    }
}
