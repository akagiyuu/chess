use super::kind::Kind;
use bevy::prelude::*;

#[derive(Debug)]
pub struct PieceMesh {
    pub cached: Vec<Handle<bevy::prelude::Mesh>>,
    pub align: Vec3,
    pub kind: Kind,
}

impl PieceMesh {
    fn new_queen(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh7/Primitive0"),
            ],
            align: Vec3::new(-0.2, 0., -0.95),
            kind: Kind::Queen,
        }
    }
    fn new_rook(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh5/Primitive0"),
            ],
            align: Vec3::new(-0.1, 0., 1.8),
            kind: Kind::Rook,
        }
    }
    fn new_bishop(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh6/Primitive0"),
            ],
            align: Vec3::new(-0.1, 0., 0.),
            kind: Kind::Bishop,
        }
    }
    fn new_pawn(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh2/Primitive0"),
            ],
            align: Vec3::new(-0.2, 0., 2.6),
            kind: Kind::Pawn,
        }
    }
    fn new_king(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
                asset_server.load("models/pieces.glb#Mesh1/Primitive0"),
            ],
            align: Vec3::new(-0.2, 0., -1.9),
            kind: Kind::King,
        }
    }
    fn new_knight(asset_server: &Res<AssetServer>) -> Self {
        Self {
            cached: vec![
                asset_server.load("models/pieces.glb#Mesh3/Primitive0"),
                asset_server.load("models/pieces.glb#Mesh4/Primitive0"),
            ],
            align: Vec3::new(-0.2, 0., 0.9),
            kind: Kind::Knight,
        }
    }
    pub fn new(asset_server: &Res<AssetServer>, kind: Kind) -> Self {
        match kind {
            Kind::King => Self::new_king(asset_server),
            Kind::Queen => Self::new_queen(asset_server),
            Kind::Rook => Self::new_rook(asset_server),
            Kind::Knight => Self::new_knight(asset_server),
            Kind::Bishop => Self::new_bishop(asset_server),
            Kind::Pawn => Self::new_pawn(asset_server)
        }
    }
}
