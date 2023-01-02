pub mod board;
pub mod pieces;

use bevy::prelude::*;

fn init(
    mut commmands: Commands,
) {
    commmands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
    });
    commmands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Tell the asset server to watch for asset changes on disk:
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "test".to_string(),
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_startup_system(init)
        .add_startup_system(board::init)
        .add_startup_system(pieces::init)
        .run();
}
