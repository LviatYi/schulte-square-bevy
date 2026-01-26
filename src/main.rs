mod gameplay;

use crate::gameplay::SchulteViewPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SchulteViewPlugin)
        .run();
}
