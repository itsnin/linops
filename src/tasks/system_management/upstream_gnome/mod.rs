// upstream_gnome task
// install vanilla gnome-core gdm ghostty network-manager xdg-user-dirs
use crate::core::task::{Category, Task};

pub mod actions;
pub mod keys;
pub mod presets;
pub mod search;
pub mod state;
pub mod ui;

impl Default for UpstreamGnome {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UpstreamGnome {
    state: state::State,
}

impl UpstreamGnome {
    pub fn new() -> Self {
        Self {
            state: state::State::new(),
        }
    }
}

impl Task for UpstreamGnome {
    fn id(&self) -> &str {
        "upstream_gnome"
    }
    fn name(&self) -> &str {
        "Upstream GNOME"
    }
    fn description(&self) -> &str {
        "install vanilla gnome-core gdm ghostty network-manager xdg-user-dirs"
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
