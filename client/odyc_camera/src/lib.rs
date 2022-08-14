use bevy::prelude::*;
use dolly::{prelude::*, rig::CameraRig};
use odyc_player::PlayerTag;

#[derive(Default)]
pub struct OdyCCameraPlugin;

impl Plugin for OdyCCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(example_system)
            .add_startup_system(setup)
            .add_system(drive_camera);
    }
}

fn example_system() {
    println!("Hello OdyCCamera");
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let camera: CameraRig<RightHanded> = CameraRig::builder()
        .with(Position::new(Vec3::ZERO))
        .with(Rotation::new(Quat::IDENTITY))
        .with(Smooth::new_position(1.5).predictive(true))
        .with(Arm::new(Vec3::new(0.0, 15.0, 10.0)))
        .with(Smooth::new_position(2.5))
        .with(
            LookAt::new(Vec3::new(0.0, 0.0, 0.0))
                .tracking_smoothness(0.5)
                .tracking_predictive(false),
        )
        .build();
    commands.insert_resource(camera);
}

fn drive_camera(
    time: Res<Time>,
    query: Query<&Transform, With<PlayerTag>>,
    mut query_cam: Query<&mut Transform, (With<Camera>, Without<PlayerTag>)>,
    mut camera_rig: ResMut<CameraRig>,
) {
    if let Some(player_transform) = query.iter().next() {
        camera_rig.driver_mut::<Position>().position =
            player_transform.translation;
        camera_rig.driver_mut::<Rotation>().rotation =
            player_transform.rotation;
        camera_rig.driver_mut::<LookAt>().target =
            player_transform.translation + Vec3::Y;
    }
    camera_rig.update(time.delta_seconds());
    if let Some(mut camera_transform) = query_cam.iter_mut().next() {
        camera_transform.translation = camera_rig.final_transform.position;
        camera_transform.rotation = camera_rig.final_transform.rotation;
    }
}
