use super::bundle::CellBundle;
use super::resource::CellMaterial;
use bevy::prelude::*;
use bevy_mod_picking::{Highlighting, PickableBundle};

pub fn spawn_squares(
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
                    ..Default::default()
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
