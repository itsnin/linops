// gnome_ext task
use crate::core::task::{Category, Task};

pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

pub struct GnomeExt {
    state: state::State,
}

impl Default for GnomeExt {
    fn default() -> Self {
        Self::new()
    }
}

impl GnomeExt {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for GnomeExt {
    fn id(&self) -> &str {
        "gnome_ext"
    }
    fn name(&self) -> &str {
        "GNOME Extensions"
    }
    fn description(&self) -> &str {
        "install gnome shell extension tools"
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
