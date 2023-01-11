use super::resource::PieceMaterial;
use super::resource::PieceMesh;
use super::startup::{assign_board_matrix, spawn_pieces};
use bevy::prelude::*;

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PieceMaterial>()
            .init_resource::<PieceMesh>()
            .add_startup_system(spawn_pieces)
            .add_startup_system(assign_board_matrix);
    }
}
