mod gameplay;

use crate::gameplay::SchulteViewPlugin;
use crate::gameplay::timer::TimerViewPlugin;
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_plugins(SchulteViewPlugin)
        .add_plugins(TimerViewPlugin)
        .run();
}
