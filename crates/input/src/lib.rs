use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
    window::PrimaryWindow,
};
use swarm_camera::camera::types::{CameraMoveEvent, CameraSet};
use swarm_core::schedule::InputSchedule;

pub fn plugin(app: &mut App) {
    app.add_systems(
        InputSchedule,
        (
            move_camera_arrow_system.before(CameraSet::MoveEvent),
            move_camera_mouse_system.before(CameraSet::MoveEvent),
        ),
    );
}

// TODO: handle multiple inputs at the same time
fn move_camera_arrow_system(
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

// TODO: make it use angles
fn move_camera_mouse_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut was_moving: Local<bool>,
    mut move_events: EventWriter<CameraMoveEvent>,
) {
    const MOVE_MARGIN: f32 = 10.0;

    let Ok(window) = window_query.single() else {
        return;
    };

    let Some(cursor) = window.cursor_position() else {
        if *was_moving {
            *was_moving = false;
            move_events.write(CameraMoveEvent(Vec2::ZERO));
        }

        return;
    };

    let mut movement = Vec2::ZERO;
    if cursor.x < MOVE_MARGIN {
        movement.x += 1.0;
    } else if cursor.x > (window.width() - MOVE_MARGIN) {
        movement.x -= 1.0;
    }

    if cursor.y < MOVE_MARGIN {
        movement.y += 1.0;
    } else if cursor.y > (window.height() - MOVE_MARGIN) {
        movement.y -= 1.0;
    }

    if (movement != Vec2::ZERO) == *was_moving {
        return;
    }
    *was_moving = movement != Vec2::ZERO;

    move_events.write(CameraMoveEvent(movement));
}
