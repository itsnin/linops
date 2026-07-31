// dev_toolchain task
// install python c c++ rust java nodejs db clients web tooling
use crate::core::task::{Category, Task};

pub mod actions;
pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

impl Default for DevToolchain {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DevToolchain {
    state: state::State,
}

impl DevToolchain {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for DevToolchain {
    fn id(&self) -> &str {
        "dev_toolchain"
    }
    fn name(&self) -> &str {
        "Dev Toolchain"
    }
    fn description(&self) -> &str {
        "install python c c++ rust java nodejs db clients web tooling"
    }
    fn category(&self) -> Category {
        Category::DevelopmentToolchain
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
