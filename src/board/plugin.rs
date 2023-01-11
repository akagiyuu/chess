use bevy::prelude::*;
use super::resource::*;
use super::startup::spawn_squares;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CellMaterial>()
            .init_resource::<BoardMatrix>()
            .add_startup_system(spawn_squares);
    }
}

