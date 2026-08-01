// gnome_debloat task
// remove 23 gnome utility apps and ptyxis
use crate::core::task::{Category, Task};

pub mod actions;
pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

impl Default for GnomeDebloat {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GnomeDebloat {
    state: state::State,
}

impl GnomeDebloat {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for GnomeDebloat {
    fn id(&self) -> &str {
        "gnome_debloat"
    }
    fn name(&self) -> &str {
        "GNOME Debloat"
    }
    fn description(&self) -> &str {
        "remove 23 gnome utility apps and ptyxis"
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
