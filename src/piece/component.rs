use bevy::prelude::Component;

#[derive(Component, Clone, Default, Debug)]
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
pub enum PieceSide {
    #[default]
    White,
    Black,
}

#[derive(Component, Clone, Default, Debug)]
pub struct Piece;
