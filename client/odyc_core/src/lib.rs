use bevy::{app::PluginGroupBuilder, prelude::*};

use odyc_camera::OdyCCameraPlugin;
use odyc_player::OdyCPlayerPlugin;
use odyc_ui::OdyCUiPlugin;
use odyc_world::OdyCWorldPlugin;

pub use odyc_camera as camera;
pub use odyc_player as player;
pub use odyc_ui as ui;
pub use odyc_world as world;

pub struct OdyCPlugins;

impl PluginGroup for OdyCPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(OdyCCameraPlugin::default());
        group.add(OdyCPlayerPlugin::default());
        group.add(OdyCUiPlugin::default());
        group.add(OdyCWorldPlugin::default());
    }
}
