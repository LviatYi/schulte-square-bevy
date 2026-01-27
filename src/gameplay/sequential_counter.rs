use crate::gameplay::level_progress_tracker::{CheckResult, LevelProgressTracker};
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

    fn max_level(&self) -> LevelSize {
        self.max_level
    }

    fn check_cell(&mut self, cell_index: LevelSize) -> CheckResult {
        match self.current_level {
            level if cell_index == level + 1 => {
                self.current_level += 1;
                CheckResult::Correct
            }
            level if cell_index <= level => CheckResult::Visited,
            _ => CheckResult::Incorrect,
        }
    }
}
