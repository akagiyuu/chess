use  bevy::prelude::Bundle;
use super::component::{PieceSide, PieceKind, Piece};

#[derive(Bundle, Clone, Default, Debug)]
pub struct PieceBundle {
    pub kind: PieceKind,
    pub side: PieceSide,

    pub _piece: Piece,
}

