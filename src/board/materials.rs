use bevy::prelude::*;

#[derive(Resource)]
pub struct CellMaterial {
    pub black: Handle<StandardMaterial>,
    pub white: Handle<StandardMaterial>,
    pub highlight: Handle<StandardMaterial>,
    pub selected: Handle<StandardMaterial>,
}

impl FromWorld for CellMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();

        Self {
            black: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            white: materials.add(Color::rgb(0., 0.1, 0.1).into()),
            highlight: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
            selected: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
        }
    }
}
