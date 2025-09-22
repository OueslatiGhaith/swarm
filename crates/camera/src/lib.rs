use bevy::prelude::*;

pub mod camera;

pub fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin,));
}
