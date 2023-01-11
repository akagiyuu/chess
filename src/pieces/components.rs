use bevy::prelude::Component;

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

#[derive(Component, Clone, Default, Debug)]
pub struct Piece;
