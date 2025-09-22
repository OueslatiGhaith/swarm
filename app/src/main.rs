use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Swarm".into(),
                mode: WindowMode::Windowed,             // TODO: change
                present_mode: PresentMode::AutoNoVsync, // TODO: change to default
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            swarm_camera::plugin,
            #[cfg(feature = "debug")]
            swarm_debug::plugin,
            swarm_input::plugin,
        ))
        .add_systems(Startup, setup_test_scene) // TODO: remove
        .run();
}

fn setup_test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
