use crate::components::Position;
use bevy::prelude::*;

#[derive(Bundle, Clone)]
pub struct CellBundle {
    pub position: Position
}

impl CellBundle {
    fn is_white(&self) -> bool {
        ((self.position.x + self.position.y)as i32) % 2 == 0
    }
}

#[derive(Default, Resource)]
pub struct SelectedCell {
    pub entity: Option<Entity>,
}
