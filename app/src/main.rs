use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            swarm_camera::plugin,
            #[cfg(feature = "debug")]
            swarm_debug::plugin,
            swarm_input::plugin,
        ))
        .run();
}
