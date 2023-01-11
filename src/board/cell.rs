use crate::components::Position;
use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct Cell;

#[derive(Bundle, Clone, Default)]
pub struct CellBundle {
    pub position: Position,
    pub _cell: Cell
}
