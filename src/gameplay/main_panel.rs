use crate::plugin::ui_host_provider_plugin::{BuiltInUiLayer, UiRootRes};
use bevy::prelude::FlexDirection::Column;
use bevy::prelude::{
    AlignItems, Commands, Component, Display, Entity, JustifyContent, JustifyItems, Node, Res,
    default, percent,
};

#[derive(Component, Debug)]
pub struct SchulteMainPanel {
    pub timer_view_slot: Entity,
    pub gameplayer_slot: Entity,
}

pub fn build_schulte_main_panel(mut commands: Commands, r_ui_root: Res<UiRootRes>) {
    let panel = commands
        .spawn((Node {
            display: Display::Flex,
            width: percent(100),
            flex_direction: Column,
            height: percent(100),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },))
        .id();

    commands
        .entity(r_ui_root.get_built_in_layer_node(BuiltInUiLayer::Main))
        .add_child(panel);

    let timer_view_slot = commands
        .spawn(Node {
            width: percent(100),
            height: percent(20),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .id();

    let gameplayer_slot = commands
        .spawn(Node {
            width: percent(100),
            height: percent(80),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .id();

    commands
        .entity(panel)
        .add_children(&[timer_view_slot, gameplayer_slot])
        .insert(SchulteMainPanel {
            timer_view_slot,
            gameplayer_slot,
        });
}
