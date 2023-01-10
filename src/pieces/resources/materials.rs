use bevy::prelude::*;

use crate::components::MeshColor;

#[derive(Resource)]
pub struct PieceMaterial {
    black: Handle<StandardMaterial>,
    white: Handle<StandardMaterial>,
}
impl FromWorld for PieceMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        Self {
            black: materials.add(Color::rgb(0., 0.2, 0.2).into()),
            white: materials.add(Color::rgb(1., 0.8, 0.8).into()),
        }
    }
}
impl PieceMaterial {
    pub fn get(&self, color: &MeshColor) -> Handle<StandardMaterial> {
        match color {
            MeshColor::White => self.black.clone(),
            MeshColor::Black => self.white.clone(),
        }
    }
}

