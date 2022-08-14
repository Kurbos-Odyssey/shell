use bevy::prelude::*;

#[derive(Default)]
pub struct OdyCUiPlugin;

impl Plugin for OdyCUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(example_system);
    }
}

fn example_system() {
    println!("Hello OdyCUI");
}
