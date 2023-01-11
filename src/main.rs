pub mod board;
pub mod game;
pub mod piece;
pub mod camera;

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(board::prelude::BoardPlugin)
        .add_plugin(piece::prelude::PiecesPlugin)
        .add_plugin(game::plugin::GamePlugin)
        .add_startup_system(camera::set_camera)
        .run();
}
