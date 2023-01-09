use bevy::prelude::{
    Bundle, Component, GlobalTransform, Handle, Mesh, PbrBundle,
    StandardMaterial, Transform, Vec3, Visibility,
};
use bevy::prelude::ComputedVisibility;
use super::components::{PieceKind, PieceSide, Position};

#[derive(Debug, Bundle, Clone)]
pub struct PieceBundle {
    pub kind: PieceKind,
    pub side: PieceSide,
    pub position: Position,
}
