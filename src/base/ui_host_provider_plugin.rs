use bevy::app::{App, Plugin};
use bevy::camera::Camera2d;
use bevy::log::trace;
use bevy::prelude::Val::Percent;
use bevy::prelude::{
    Commands, Component, Entity, IntoScheduleConfigs, Name, PositionType, Resource, Startup,
    default,
};
use bevy::ui::Node;
use std::collections::HashMap;

#[derive(bevy::prelude::SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiStartupSet {
    Bootstrap,
    GameplayView,
}

#[derive(Resource, Debug)]
pub struct UiRootRes {
    pub ui_root: Entity,
    layers: HashMap<UiLayerKey, Entity>,
}

impl UiRootRes {
    fn new(ui_root: Entity) -> Self {
        UiRootRes {
            ui_root,
            layers: HashMap::new(),
        }
    }

    pub fn get_layer_node(&self, key: &UiLayerKey) -> Option<Entity> {
        self.layers.get(key).copied()
    }

    fn pre_build_layer_node(&mut self, commands: &mut Commands, key: UiLayerKey) -> bool {
        let layer_name = format!("UiRootRes::Layer::{}", key.as_ref());
        let key_for_component = key.clone();
        let mut created_layer = None;
        commands.entity(self.ui_root).with_children(|root| {
            let layer_entity = root
                .spawn((
                    Name::new(layer_name),
                    UiLayerNode(key_for_component),
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
        self.layers.insert(key, created_layer).is_some()
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
    trace!("Bootstrapping UiHostProviderPlugin");
    commands.spawn(Camera2d);
}

fn spawn_ui_root(mut commands: Commands) {
    trace!("Spawning UiRoot Entity");
    let ui_root = commands
        .spawn((Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },))
        .id();

    let mut ui_root_res = UiRootRes::new(ui_root);
    for layer_key in UiLayerKey::builtin_layers() {
        ui_root_res.pre_build_layer_node(&mut commands, layer_key.clone());
    }

    commands.insert_resource(ui_root_res);
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiLayerNode(pub UiLayerKey);

macro_rules! define_ui_layer_key {
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            builtin: [ $( $Builtin:ident ),+ $(,)? ],
            custom: $Custom:ident ( $CustomTy:ty ) $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis enum $Name {
            $( $Builtin, )+
            $Custom($CustomTy),
        }

        impl AsRef<str> for $Name {
            fn as_ref(&self) -> &str {
                match self {
                    $( Self::$Builtin => stringify!($Builtin), )+
                    Self::$Custom(v) => v.as_ref(),
                }
            }
        }

        impl UiLayerKey {
            #[doc="Creates a custom (pascal style recommended) `UiLayerKey` with the given name."]
            pub fn custom(name: impl Into<String>) -> Self {
                Self::Custom(name.into())
            }

            const BUILTIN_LAYERS: &'static [Self] = &[
                $(Self::$Builtin,)*
            ];

            pub fn builtin_layers() -> &'static [Self] {
                &Self::BUILTIN_LAYERS
            }
        }
    };
}

define_ui_layer_key! {
    pub enum UiLayerKey {
        builtin: [ Background, Main, Debug ],
        custom: Custom(String),
    }
}
