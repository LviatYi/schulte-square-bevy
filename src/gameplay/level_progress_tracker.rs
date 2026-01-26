use crate::gameplay::LevelSize;

pub trait LevelProgressTracker {
    fn current_level(&self) -> LevelSize;

    fn is_level_completed(&self) -> bool;

    fn check_cell(&mut self, cell_index: LevelSize) -> bool;
}
