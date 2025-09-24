use bevy::prelude::*;

pub mod schedule;

pub fn plugin(app: &mut App) {
    app.add_plugins((schedule::plugin,));
}
