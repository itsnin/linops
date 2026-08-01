// system_util task
use crate::core::task::{Category, Task};

pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

pub struct SystemUtil {
    state: state::State,
}

impl Default for SystemUtil {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemUtil {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for SystemUtil {
    fn id(&self) -> &str {
        "system_util"
    }
    fn name(&self) -> &str {
        "System Utilities"
    }
    fn description(&self) -> &str {
        "install system utility tools"
    }
    fn category(&self) -> Category {
        Category::PackageManagement
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
