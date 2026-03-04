mod gameplay;
mod plugin;

use crate::gameplay::SchulteViewPlugin;
use crate::plugin::ui_host_provider_plugin::{UiHostProviderPlugin, UiStartupSet};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(UiHostProviderPlugin)
        .add_plugins(SchulteViewPlugin)
        .configure_sets(Startup, UiStartupSet::Bootstrap)
        .configure_sets(
            Startup,
            UiStartupSet::GameplayView.after(UiStartupSet::Bootstrap),
        )
        .run();
}
