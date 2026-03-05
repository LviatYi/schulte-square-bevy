mod constant;
pub mod gameplay_timer;
mod main_panel;
mod schulte_gameplay_view;
mod sequential_counter;

use crate::gameplay::constant::{
    CORRECT_START_COLOR, DEFAULT_BUTTON_COLOR, DISABLED_BUTTON_COLOR, HOVERED_BUTTON_COLOR,
    INCORRECT_START_COLOR, PRESSED_BUTTON_COLOR,
};
use crate::gameplay::gameplay_timer::{
    GameplayTimer, build_gameplay_timer_view, update_gameplay_timer_view,
};
use crate::gameplay::main_panel::build_schulte_main_panel;
use crate::gameplay::schulte_gameplay_view::build_gameplay_schulte_view;
use crate::gameplay::sequential_counter::{CheckResult, SequentialCounter};
use crate::plugin::ui_host_provider_plugin::UiStartupSet;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::log::info;
use bevy::prelude::{
    BackgroundColor, Changed, Commands, Component, EaseFunction, Entity, Interaction,
    IntoScheduleConfigs, Query, ResMut,
};
use bevy_tweening::{Tween, TweenAnim};

type LevelSize = u8;

const GRID_SIZE: LevelSize = 3;

pub struct SchultePlugin;

impl Plugin for SchultePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                build_schulte_main_panel,
                build_gameplay_schulte_view,
                build_gameplay_timer_view,
            )
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
