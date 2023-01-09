use bevy::prelude::Component;

#[derive(Debug, Default, Clone, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
