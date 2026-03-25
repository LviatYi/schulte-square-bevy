use crate::plugin::lynx::aspect_fit_in_slot::update_max_aspect_fit_nodes;
use bevy::app::{App, Plugin, PostUpdate};
use bevy::prelude::{IntoScheduleConfigs, Val};
use bevy::ui::UiSystems;

pub struct LynxPlugin;

impl Plugin for LynxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            update_max_aspect_fit_nodes.after(UiSystems::Layout),
        );
    }
}

pub fn same_val(current: &Val, target: &Val) -> bool {
    match (current, target) {
        (Val::Auto, Val::Auto) => true,
        (Val::Percent(current), Val::Percent(target)) => (current - target).abs() <= 0.01,
        (Val::Px(current), Val::Px(target)) => (current - target).abs() <= 0.5,
        _ => false,
    }
}
