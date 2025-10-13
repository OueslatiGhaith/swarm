use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraSet {
    MoveEvent,
}

#[derive(Default, Resource)]
pub struct CameraMovement {
    movement: Vec2,
}

impl CameraMovement {
    pub(crate) fn movement(&self) -> Vec2 {
        self.movement
    }

    pub(crate) fn set(&mut self, movement: Vec2) {
        self.movement = movement;
    }
}

#[derive(Component)]
pub struct CameraRig;

#[derive(Message)]
pub struct CameraMoveEvent(pub Vec2);
