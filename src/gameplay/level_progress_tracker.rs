use crate::gameplay::LevelSize;

pub enum CheckResult {
    Correct,
    Visited,
    Incorrect,
}

pub trait LevelProgressTracker {
    fn current_level(&self) -> LevelSize;

    fn max_level(&self) -> LevelSize;

    fn visited(&self, cell_index: LevelSize) -> bool {
        cell_index <= self.current_level()
    }

    fn is_level_completed(&self) -> bool {
        self.current_level() >= self.max_level()
    }

    fn check_cell(&mut self, cell_index: LevelSize) -> CheckResult;
}
