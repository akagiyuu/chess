use super::event::MoveEvent;
use super::resource::SelectedCell;
use crate::board::prelude::Square;
use crate::piece::prelude::Piece;
use bevy::prelude::*;
use bevy_mod_picking::PickingCamera;

pub fn select_cell(
    mouse_input: Res<Input<MouseButton>>,
    mut selected_cell: ResMut<SelectedCell>,
    mut move_event: EventWriter<MoveEvent>,
    cell_query: Query<&Transform, With<Square>>,
    camera_query: Query<&PickingCamera>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(picking_camera) = camera_query.iter().last() else {
        return;
    };

    let Some((new_selected, _)) = picking_camera.get_nearest_intersection() else {
        selected_cell.entity = None;
        return;
    };

    selected_cell.entity = selected_cell
        .entity
        .map_or(Some(new_selected), |old_selected| {
            if let Ok([old_transform, new_transform]) =
                cell_query.get_many([old_selected, new_selected])
            {
                move_event.send(MoveEvent {
                    source: old_transform.translation,
                    destination: new_transform.translation,
                });
            };

            None
        })
}

pub fn move_piece(
    mut move_event: EventReader<MoveEvent>,
    mut query: Query<&mut Transform, With<Piece>>,
) {
    for event in move_event.iter() {
        for mut piece_transform in query.iter_mut() {
            if piece_transform.translation != event.source {
                continue;
            }
            piece_transform.translation = event.destination;
        }
    }
}
