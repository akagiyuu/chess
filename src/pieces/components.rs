use bevy::prelude::Component;
use crate::components::Position;

#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy, Component)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    #[default]
    Pawn,
}

#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy, Component)]
pub enum PieceSide {
    #[default]
    Black,
    White,
}

