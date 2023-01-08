use bevy::prelude::{
    Bundle, Component, ComputedVisibility, GlobalTransform, Handle, Mesh, PbrBundle,
    StandardMaterial, Transform, Vec3, Visibility,
};

#[derive(Debug, Default, Clone, Copy, Component)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    #[default]
    Pawn,
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub enum PieceSide {
    #[default]
    Black,
    White,
}

#[derive(Bundle, Clone)]
pub struct PieceBundle {
    kind: PieceKind,
    side: PieceSide,
    x: f32,
    y: f32,
}
