pub mod gameplay_timer;
mod sequential_counter;

use crate::gameplay::gameplay_timer::{
    GameplayTimer, build_gameplay_timer_view, update_gameplay_timer_view,
};
use crate::gameplay::sequential_counter::{CheckResult, SequentialCounter};
use crate::plugin::ui_host_provider_plugin::{BuiltInUiLayer, UiRootRes, UiStartupSet};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::log::info;
use bevy::prelude::{
    AlignItems, BackgroundColor, Button, Changed, Commands, Component, Display, EaseFunction,
    Entity, Interaction, IntoScheduleConfigs, JustifyContent, JustifyItems, Node, Query,
    RepeatedGridTrack, Res, ResMut, Text, UiRect, default, percent, px,
};
use bevy_tweening::{Tween, TweenAnim};
use rand::prelude::SliceRandom;

const DEFAULT_BUTTON_COLOR: Color = Color::srgb(0.85, 0.53, 0.54);
const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
const DISABLED_BUTTON_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

const CORRECT_START_COLOR: Color = Color::srgb(0.21, 0.36, 0.22);
const INCORRECT_START_COLOR: Color = Color::srgb(0.69, 0.32, 0.36);

type LevelSize = u8;

const GRID_SIZE: LevelSize = 3;

pub struct SchulteViewPlugin;

impl Plugin for SchulteViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (build_schulte_view, build_gameplay_timer_view)
                .chain()
                .in_set(UiStartupSet::GameplayView),
        );
        app.add_systems(Update, handle_cell_click);
        app.add_systems(Update, handle_cell_hover);
        app.add_systems(Update, update_gameplay_timer_view);
    }
}

#[derive(Component, Debug, Copy, Clone)]
struct CellIndex(LevelSize);

fn build_schulte_view(mut commands: Commands, ui_root_res: Res<UiRootRes>) {
    commands
        .entity(ui_root_res.get_built_in_layer_node(BuiltInUiLayer::Main))
        .with_children(|root| {
            root.spawn(Node {
                display: Display::Grid,
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                ..default()
            })
            .with_children(|root| {
                root.spawn((
                    Node {
                        display: Display::Grid,
                        height: px(400),
                        aspect_ratio: Some(1.0),
                        grid_template_columns: vec![RepeatedGridTrack::flex(GRID_SIZE as u16, 1.0)],
                        grid_template_rows: vec![RepeatedGridTrack::flex(GRID_SIZE as u16, 1.0)],
                        row_gap: px(8),
                        column_gap: px(8),
                        padding: UiRect::all(px(8)),
                        align_items: AlignItems::Stretch,
                        justify_items: JustifyItems::Stretch,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.24, 0.24, 0.24)),
                ))
                .with_children(|grid| {
                    let indexes = (1..=(GRID_SIZE * GRID_SIZE)).collect::<Vec<LevelSize>>();
                    let shuffled_indexes = {
                        let mut v = indexes.clone();
                        let mut rng = rand::rng();
                        v.as_mut_slice().shuffle(&mut rng);
                        v
                    };
                    for i in 0..(GRID_SIZE * GRID_SIZE) {
                        let cell_index = shuffled_indexes[i as usize];
                        grid.spawn((
                            Button,
                            CellIndex(cell_index),
                            Node {
                                width: percent(100.0),
                                height: percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(DEFAULT_BUTTON_COLOR),
                        ))
                        .with_children(|btn| {
                            btn.spawn((Text::new(format!("{}", cell_index)),));
                        });
                    }
                });
            });
        });

    commands.insert_resource(SequentialCounter::new(GRID_SIZE * GRID_SIZE));
}

fn handle_cell_click(
    mut commands: Commands,
    mut tracker: ResMut<SequentialCounter>,
    mut q: Query<(Entity, &CellIndex, &Interaction), Changed<Interaction>>,
    mut timer_q: Query<&mut GameplayTimer>,
) {
    let mut timer = timer_q.single_mut();

    for (e, idx, interaction) in &mut q {
        if *interaction == Interaction::Pressed {
            match tracker.check_cell(idx.0) {
                CheckResult::Correct { is_first } => {
                    if is_first {
                        info!("First cell clicked: {}", idx.0);

                        if let Ok(timer) = timer.as_mut() {
                            timer.reset().resume();
                        }
                    } else {
                        info!("Correct cell clicked: {}", idx.0);
                    }

                    let tween = Tween::new(
                        bevy_tweening::EaseMethod::EaseFunction(EaseFunction::CubicIn),
                        std::time::Duration::from_secs_f32(0.5),
                        bevy_tweening::lens::UiBackgroundColorLens {
                            start: CORRECT_START_COLOR,
                            end: DISABLED_BUTTON_COLOR,
                        },
                    );

                    commands.entity(e).insert(TweenAnim::new(tween));
                }
                CheckResult::Incorrect => {
                    info!("Incorrect cell clicked: {}", idx.0);
                    info!("You should click: {}", tracker.current_level() + 1);

                    let tween = Tween::new(
                        bevy_tweening::EaseMethod::EaseFunction(EaseFunction::CubicIn),
                        std::time::Duration::from_secs_f32(0.5),
                        bevy_tweening::lens::UiBackgroundColorLens {
                            start: INCORRECT_START_COLOR,
                            end: DEFAULT_BUTTON_COLOR,
                        },
                    );

                    commands.entity(e).insert(TweenAnim::new(tween));
                }
                _ => {}
            }

            if tracker.is_level_completed() {
                info!("Level completed!");
                if let Ok(timer) = timer.as_mut() {
                    timer.pause();
                    info!("Cost time: {:.2} seconds", timer.elapsed().as_secs_f32());
                }
            }
        }
    }
}

fn handle_cell_hover(
    tracker: ResMut<SequentialCounter>,
    mut q: Query<(&CellIndex, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (cell, interaction, mut color) in &mut q {
        if tracker.visited(cell.0) {
            continue;
        }
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON_COLOR);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *color = BackgroundColor(DEFAULT_BUTTON_COLOR);
            }
        }
    }
}
