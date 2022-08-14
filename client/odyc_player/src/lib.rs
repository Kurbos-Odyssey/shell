use bevy::prelude::*;

#[derive(Default)]
pub struct OdyCPlayerPlugin;

#[derive(Component)]
pub struct PlayerTag;

const SPEED: f32 = 5.0;

impl Plugin for OdyCPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(example_system)
            .add_startup_system(setup)
            .add_system(handle_input);
    }
}

fn example_system() {
    println!("Hello OdyCPlayer");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(PlayerTag);
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerTag>>,
) {
    if let Some(mut player_transform) = query.iter_mut().next() {
        let mut velocity = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            velocity -= Vec3::Z;
        }
        if keys.pressed(KeyCode::A) {
            velocity -= Vec3::X;
        }
        if keys.pressed(KeyCode::S) {
            velocity += Vec3::Z;
        }
        if keys.pressed(KeyCode::D) {
            velocity += Vec3::X;
        }

        let velocity =
            velocity.normalize_or_zero() * SPEED * time.delta_seconds();

        player_transform.translation += velocity;
    }
}
