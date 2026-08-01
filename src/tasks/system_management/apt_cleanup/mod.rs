// apt_cleanup task
// mark manual drop metapackages autoremove purge
use crate::core::task::{Category, Task};

pub mod actions;
pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

impl Default for AptCleanup {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AptCleanup {
    state: state::State,
}

impl AptCleanup {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for AptCleanup {
    fn id(&self) -> &str {
        "apt_cleanup"
    }
    fn name(&self) -> &str {
        "APT Cleanup"
    }
    fn description(&self) -> &str {
        "mark manual drop metapackages autoremove purge"
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
