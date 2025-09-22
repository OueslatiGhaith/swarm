use std::time::Duration;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    diagnostic::LogDiagnosticsPlugin,
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont::default().with_font_size(10.0),
                ..Default::default()
            },
        },
        LogDiagnosticsPlugin {
            debug: false,
            wait_duration: Duration::from_secs(10),
            filter: None,
        },
    ));
}
