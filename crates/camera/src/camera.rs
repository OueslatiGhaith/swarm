use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
