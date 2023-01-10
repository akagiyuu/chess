pub mod cell;
pub mod materials;

use crate::components::Position;
use bevy::prelude::*;
use bevy_mod_picking::{
    CustomHighlightPlugin, DefaultHighlighting, DefaultPickingPlugins, Highlighting, HoverEvent,
    PickableBundle, PickingEvent, SelectionEvent,
};
use cell::CellBundle;
use materials::CellMaterial;

use self::cell::SelectedCell;

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
                Highlighting {
                    initial: if (i + j) % 2 == 0 {
                        materials.white.clone()
                    } else {
                        materials.black.clone()
                    },
                    hovered: Some(materials.highlight.clone()),
                    selected: Some(materials.selected.clone()),
                    pressed: None,
                },
            ));
        }
    }
}

pub fn select_cell(
    mut events: EventReader<PickingEvent>,
    mut selected_cell: ResMut<SelectedCell>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(e) = event {
            selected_cell.entity = match e {
                SelectionEvent::JustSelected(entity) => Some(*entity),
                SelectionEvent::JustDeselected(_) => None,
            };
        }
    }
}
pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCell>()
            .init_resource::<CellMaterial>()
            .add_plugins(DefaultPickingPlugins)
            .add_startup_system(init);
        // .add_system_to_stage(CoreStage::PostUpdate, test);
    }
}
