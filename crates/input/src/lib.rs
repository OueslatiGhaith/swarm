use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};
use swarm_camera::camera::CameraMoveEvent;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, move_camera_arrow_system);
}

// TODO: lerp
pub fn move_camera_arrow_system(
    mut key_events: EventReader<KeyboardInput>,
    mut move_events: EventWriter<CameraMoveEvent>,
) {
    for key_event in key_events.read() {
        let mut direction = Vec2::ZERO;

        if key_event.key_code == KeyCode::ArrowLeft {
            direction = Vec2::new(1., 0.);
        } else if key_event.key_code == KeyCode::ArrowRight {
            direction = Vec2::new(-1., 0.);
        } else if key_event.key_code == KeyCode::ArrowDown {
            direction = Vec2::new(0., -1.);
        } else if key_event.key_code == KeyCode::ArrowUp {
            direction = Vec2::new(0., 1.);
        }

        if direction == Vec2::ZERO {
            continue;
        }
        if key_event.state == ButtonState::Released {
            direction = Vec2::ZERO;
        }

        move_events.write(CameraMoveEvent(direction));
    }
}
