// editor task
use crate::core::task::{Category, Task};

pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

pub struct Editor {
    state: state::State,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for Editor {
    fn id(&self) -> &str {
        "editor"
    }
    fn name(&self) -> &str {
        "Editor"
    }
    fn description(&self) -> &str {
        "install text editors"
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
