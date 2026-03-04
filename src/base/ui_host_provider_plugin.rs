use bevy::app::{App, Plugin};
use bevy::camera::Camera2d;
use bevy::log::{debug, info, trace};
use bevy::prelude::Val::Percent;
use bevy::prelude::{
    Commands, Component, ContainsEntity, Entity, IntoScheduleConfigs, Name, PositionType, Resource,
    Startup, default,
};
use bevy::ui::Node;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(bevy::prelude::SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiStartupSet {
    Bootstrap,
    GameplayView,
}

#[derive(Resource, Debug)]
pub struct UiRootRes {
    ui_root: Entity,

    layers: HashMap<UiLayerKey, Entity>,
}

impl UiRootRes {
    fn new(ui_root: Entity) -> Self {
        UiRootRes {
            ui_root,
            layers: HashMap::new(),
        }
    }

    pub fn get_built_in_layer_node(&self, key: BuiltInUiLayer) -> Entity {
        match self.layers.get(&key.into()) {
            None => {
                panic!(
                    "built-in layer node for key {:?} should have been created during bootstrap",
                    key
                );
            }
            Some(e) => e.entity(),
        }
    }

    pub fn try_get_layer_node(&self, key: &UiLayerKey) -> Option<Entity> {
        self.layers.get(key).copied()
    }

    pub fn get_layer_node(&self, key: &UiLayerKey) -> Entity {
        self.try_get_layer_node(key).unwrap_or_else(|| {
            panic!(
                "layer node for key {:?} should have been created during bootstrap",
                key
            )
        })
    }

    fn pre_build_layer_node(&mut self, commands: &mut Commands, key: BuiltInUiLayer) -> bool {
        let layer_name = format!("UiRootRes::Layer::{}", key.as_ref());
        let mut created_layer = None;
        commands.entity(self.ui_root).with_children(|root| {
            let layer_entity = root
                .spawn((
                    Name::new(layer_name),
                    UiLayerNode(key.into()),
                    Node {
                        width: Percent(100.0),
                        height: Percent(100.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                ))
                .id();
            created_layer = Some(layer_entity);
        });

        let created_layer = created_layer.expect("built-in layer node should always be created");
        self.layers.insert(key.into(), created_layer).is_some()
    }
}

pub struct UiHostProviderPlugin;

impl Plugin for UiHostProviderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (bootstrap, spawn_ui_root)
                .chain()
                .in_set(UiStartupSet::Bootstrap),
        );
    }
}

fn bootstrap(mut commands: Commands) {
    debug!("Bootstrapping UiHostProviderPlugin");
    commands.spawn(Camera2d);
}

fn spawn_ui_root(mut commands: Commands) {
    debug!("Spawning UiRoot Entity");
    let ui_root = commands
        .spawn((Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },))
        .id();

    let mut ui_root_res = UiRootRes::new(ui_root);
    for layer_key in BuiltInUiLayer::iter() {
        ui_root_res.pre_build_layer_node(&mut commands, layer_key);
    }

    commands.insert_resource(ui_root_res);
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiLayerNode(pub UiLayerKey);

#[derive(Debug, Copy, Clone, strum::EnumIter, strum::AsRefStr, PartialEq, Eq, Hash)]
pub enum BuiltInUiLayer {
    Background,
    Main,
    Debug,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UiLayerKey {
    BuiltIn(BuiltInUiLayer),
    Custom(String),
}

impl From<BuiltInUiLayer> for UiLayerKey {
    fn from(value: BuiltInUiLayer) -> Self {
        UiLayerKey::BuiltIn(value)
    }
}

impl AsRef<str> for UiLayerKey {
    fn as_ref(&self) -> &str {
        match self {
            UiLayerKey::BuiltIn(builtin) => builtin.as_ref(),
            UiLayerKey::Custom(v) => v.as_ref(),
        }
    }
}

impl UiLayerKey {
    /// Creates a custom (pascal style recommended) `UiLayerKey` with the given name.
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom(name.into())
    }
}
