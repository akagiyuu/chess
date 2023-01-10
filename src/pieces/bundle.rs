use super::components::PieceKind;
use crate::components::{Position, MeshColor};
use bevy::prelude::Bundle;

#[derive(Debug, Bundle, Clone)]
pub struct PieceBundle {
    pub kind: PieceKind,
    pub color: MeshColor,
    pub position: Position,
}
