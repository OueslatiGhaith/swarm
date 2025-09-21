use bevy::prelude::*;

pub mod fps;

pub fn plugin(app: &mut App) {
    app.add_plugins((fps::plugin,));
}
