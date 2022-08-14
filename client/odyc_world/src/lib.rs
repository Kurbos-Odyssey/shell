use bevy::prelude::*;

#[derive(Default)]
pub struct OdyCWorldPlugin;

impl Plugin for OdyCWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(example_system)
            .add_startup_system(setup);
    }
}

fn example_system() {
    println!("Hello OdyCWorld");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
