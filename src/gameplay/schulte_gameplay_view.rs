use crate::gameplay::constant::GRID_CONTAINER_COLOR;
use crate::gameplay::main_panel::SchulteMainPanel;
use crate::gameplay::sequential_counter::SequentialCounter;
use crate::gameplay::{CellIndex, DEFAULT_BUTTON_COLOR, GRID_SIZE, LevelSize};
use bevy::prelude::{
    AlignItems, BackgroundColor, Button, Commands, Display, JustifyContent, JustifyItems, Node,
    RepeatedGridTrack, Single, Text, UiRect, default, percent, px,
};
use rand::prelude::SliceRandom;

pub fn build_gameplay_schulte_view(mut commands: Commands, main_panel: Single<&SchulteMainPanel>) {
    commands
        .entity(main_panel.gameplayer_slot)
        .with_children(|root| {
            // Grid container
            root.spawn((
                Node {
                    display: Display::Grid,
                    height: percent(100),
                    width: percent(100),
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
                BackgroundColor(GRID_CONTAINER_COLOR),
            ))
            .with_children(|grid| {
                // Cells
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

    commands.insert_resource(SequentialCounter::new(GRID_SIZE * GRID_SIZE));
}
