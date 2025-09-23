use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_event::<CameraMoveEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, move_cam);
}

#[derive(Event)]
pub struct CameraMoveEvent(pub Vec2);

#[derive(Component)]
pub struct CameraRig;

pub fn setup(mut commands: Commands) {
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

pub fn move_cam(
    mut camera_move_events: EventReader<CameraMoveEvent>,
    mut query: Query<&mut Transform, With<CameraRig>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    for event in camera_move_events.read() {
        transform.translation.x += event.0.x;
        transform.translation.z += event.0.y;
    }
}
