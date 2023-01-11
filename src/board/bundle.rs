use bevy::prelude::Bundle;
use super::component::Square;

#[derive(Bundle, Clone, Default)]
pub struct CellBundle {
    pub _square: Square
}
