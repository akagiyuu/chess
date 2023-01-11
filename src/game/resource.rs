use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SelectedCell {
    pub entity: Option<Entity>,
}
