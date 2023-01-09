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

#[derive(Default)]
struct SelectedCell {
    entity: Option<Entity>,
}
