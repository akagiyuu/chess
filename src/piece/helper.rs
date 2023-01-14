use crate::{board::resource::BoardMatrix, game::event::MoveEvent};

use super::component::PieceKind;

pub fn is_valid_move(move_event: &MoveEvent, board_matrix: &BoardMatrix, kind: PieceKind) -> bool {
    true
}

fn is_valid_queen_move(move_event: &MoveEvent, board_matrix: &BoardMatrix) -> bool {
    true
}
