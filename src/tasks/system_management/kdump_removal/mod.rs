// kdump_removal task
// remove kdump-tools to free 512mb reserved memory
use crate::core::task::{Category, Task};

pub mod actions;
pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

impl Default for KdumpRemoval {
    fn default() -> Self {
        Self::new()
    }
}

pub struct KdumpRemoval {
    state: state::State,
}

impl KdumpRemoval {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for KdumpRemoval {
    fn id(&self) -> &str {
        "kdump_removal"
    }
    fn name(&self) -> &str {
        "Kdump Removal"
    }
    fn description(&self) -> &str {
        "remove kdump-tools to free 512mb reserved memory"
    }
    fn category(&self) -> Category {
        Category::SystemManagement
    }
    fn supports_distro(&self, _: &crate::core::distro::DistroId) -> bool {
        true
    }
    fn handle_key(&mut self, key: crate::core::key::Key) -> Vec<crate::core::action::Action> {
        keys::handle(&mut self.state, key)
    }
    fn render(&self, area: crate::core::rect::Rect, grid: &mut crate::core::grid::Grid) {
        ui::render(&self.state, area, grid);
    }
    fn searchable_items(&self) -> Vec<crate::core::search::SearchItem> {
        search::items(&self.state)
    }
}
