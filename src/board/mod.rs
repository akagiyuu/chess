pub mod cell;
pub mod materials;

use crate::components::Position;
use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickingEvent, SelectionEvent, DefaultPickingPlugins};
use cell::CellBundle;
use materials::CellMaterial;

pub fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<CellMaterial>,
) {
    let cell_mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    for i in 0..8 {
        for j in 0..8 {
            commands.spawn((
                PbrBundle {
                    mesh: cell_mesh.clone(),
                    material: if (i + j) % 2 == 0 {
                        materials.white.clone()
                    } else {
                        materials.black.clone()
                    },
                    transform: Transform::from_xyz(i as f32, 0., j as f32),
                    ..Default::default()
                },
                CellBundle {
                    position: Position {
                        x: i as f32,
                        y: j as f32,
                    },
                },
                PickableBundle::default(),
            ));
        }
    }
}

// pub fn test_query(
//     mut query: Query<(Entity, &Position, &mut Handle<StandardMaterial>)>,
//     materials: Res<CellMaterial>
// ) {
//     for (entity, test, mut material) in query.iter_mut() {
//         *material = materials.white.clone();
//     }
// }
pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .init_resource::<CellMaterial>()
            .add_startup_system(init)
            .add_system_to_stage(CoreStage::PostUpdate, test);
    }
}
