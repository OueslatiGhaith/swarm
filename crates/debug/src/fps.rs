use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_plugins(FpsOverlayPlugin::default());
}
