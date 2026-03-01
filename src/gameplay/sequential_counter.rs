use crate::gameplay::LevelSize;
use bevy::prelude::Resource;

#[derive(Resource, Debug)]
pub struct SequentialCounter {
    current_level: LevelSize,

    max_level: LevelSize,
}

const START_LEVEL: LevelSize = 0;

pub enum CheckResult {
    Correct { is_first: bool },
    Visited,
    Incorrect,
}

impl SequentialCounter {
    pub fn new(max_level: LevelSize) -> Self {
        SequentialCounter {
            current_level: START_LEVEL,
            max_level,
        }
    }

    pub fn current_level(&self) -> LevelSize {
        self.current_level
    }

    pub fn max_level(&self) -> LevelSize {
        self.max_level
    }

    pub fn visited(&self, cell_index: LevelSize) -> bool {
        cell_index <= self.current_level()
    }

    pub fn is_level_completed(&self) -> bool {
        self.current_level() >= self.max_level()
    }

    pub fn check_cell(&mut self, cell_index: LevelSize) -> CheckResult {
        match self.current_level {
            level if cell_index == level + 1 => {
                let is_first = self.current_level == START_LEVEL;
                self.current_level += 1;
                CheckResult::Correct { is_first }
            }
            level if cell_index <= level => CheckResult::Visited,
            _ => CheckResult::Incorrect,
        }
    }
}
