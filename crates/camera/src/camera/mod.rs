use bevy::prelude::*;

use swarm_core::schedule::{InputSchedule, Movement};
use types::*;

pub mod types;

pub fn plugin(app: &mut App) {
    app.add_message::<CameraMoveEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            InputSchedule,
            (handle_camera_move_event.in_set(CameraSet::MoveEvent),),
        )
        .add_systems(Movement, (move_camera,));
}

fn setup(mut commands: Commands) {
    commands.insert_resource(CameraMovement::default());

    commands
        .spawn((
            Name::new("camera_rig"),
            CameraRig,
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("camera"),
                Camera3d::default(),
                Transform::from_xyz(0.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });
}

fn handle_camera_move_event(
    mut movement: ResMut<CameraMovement>,
    mut events: MessageReader<CameraMoveEvent>,
) {
    if let Some(event) = events.read().last() {
        movement.set(event.0);
    }
}

fn move_camera(
    movement: Res<CameraMovement>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<CameraRig>>,
) {
    const CAMERA_SPEED: f32 = 2.0;

    let direction = movement.movement();
    if direction == Vec2::ZERO {
        return;
    }

    let Ok(mut transform) = camera_query.single_mut() else {
        return;
    };

    let time_delta = time.delta().as_secs_f32();
    let delta_scalar = time_delta * CAMERA_SPEED;
    let delta_vec = (transform.rotation * direction.extend(0.0)) * delta_scalar;

    transform.translation.x += delta_vec.x;
    transform.translation.z += delta_vec.y;
}
