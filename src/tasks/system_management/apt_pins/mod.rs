// apt_pins task
// write apt pin files to block removed packages
use crate::core::task::{Category, Task};

pub mod actions;
pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

impl Default for AptPins {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AptPins {
    state: state::State,
}

impl AptPins {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for AptPins {
    fn id(&self) -> &str {
        "apt_pins"
    }
    fn name(&self) -> &str {
        "APT Pins"
    }
    fn description(&self) -> &str {
        "write apt pin files to block removed packages"
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
