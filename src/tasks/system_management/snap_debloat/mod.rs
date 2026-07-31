// snap_debloat task
// remove snapd and pin it from reinstalling
use crate::core::task::{Category, Task};

pub mod actions;
pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

impl Default for SnapDebloat {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SnapDebloat {
    state: state::State,
}

impl SnapDebloat {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for SnapDebloat {
    fn id(&self) -> &str {
        "snap_debloat"
    }
    fn name(&self) -> &str {
        "Snap Debloat"
    }
    fn description(&self) -> &str {
        "remove snapd and pin it from reinstalling"
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
