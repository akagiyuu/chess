use super::components::{PieceKind, Piece};
use crate::components::MeshColor;
use bevy::prelude::Bundle;

#[derive(Debug, Bundle, Clone, Default)]
pub struct PieceBundle {
    pub kind: PieceKind,
    pub color: MeshColor,

    pub _piece: Piece
}
