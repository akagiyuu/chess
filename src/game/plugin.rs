use super::event::MoveEvent;
use super::resource::SelectedCell;
use super::system::{select_cell, move_piece};
use bevy::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCell>()
            .add_event::<MoveEvent>()
            .add_system(select_cell)
            .add_system(move_piece);
    }
}
