use super::components::{PieceKind, PieceSide};
use crate::components::Position;
use bevy::prelude::Bundle;

#[derive(Debug, Bundle, Clone)]
pub struct PieceBundle {
    pub kind: PieceKind,
    pub side: PieceSide,
    pub position: Position,
}
