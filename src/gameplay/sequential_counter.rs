use crate::gameplay::level_progress_tracker::LevelProgressTracker;
use crate::gameplay::{LevelSize, START_LEVEL};
use bevy::prelude::Resource;

#[derive(Resource, Debug)]
pub struct SequentialCounter {
    current_level: LevelSize,

    max_level: LevelSize,
}

impl SequentialCounter {
    pub fn new(max_level: LevelSize) -> Self {
        SequentialCounter {
            current_level: START_LEVEL,
            max_level,
        }
    }
}

impl LevelProgressTracker for SequentialCounter {
    fn current_level(&self) -> LevelSize {
        self.current_level
    }

    fn is_level_completed(&self) -> bool {
        self.current_level >= self.max_level
    }

    fn check_cell(&mut self, cell_index: LevelSize) -> bool {
        if cell_index == self.current_level + 1 {
            self.current_level += 1;
            true
        } else {
            false
        }
    }
}
