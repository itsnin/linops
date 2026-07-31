// system_update task
// apt update and apt upgrade
use crate::core::task::{Category, Task};

pub mod actions;
pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

impl Default for SystemUpdate {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SystemUpdate {
    state: state::State,
}

impl SystemUpdate {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for SystemUpdate {
    fn id(&self) -> &str {
        "system_update"
    }
    fn name(&self) -> &str {
        "System Update"
    }
    fn description(&self) -> &str {
        "apt update and apt upgrade"
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
