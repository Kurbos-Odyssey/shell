use bevy::prelude::*;
use odyc::OdyCPlugins;

// These are unused; for demonstration of architecture
// use odyc::camera;
// use odyc::player;
// use odyc::ui;
// use odyc::world;

pub const LAUNCHER_TITLE: &str = "Odyssey";

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(OdyCPlugins)
    .add_startup_system(load_icon);
    app
}

fn load_icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bevy.png"),
        ..default()
    });
}
