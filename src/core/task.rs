// the task trait is what every task implements
// a task owns its state its ui its keys its actions its presets its search items
// tasks never call apt or dnf directly they return actions for the core to execute
// tasks never touch the system directly
//
// each task directory has these files
// mod.rs      struct plus task trait impl
// state.rs    task specific typed state
// ui.rs       task specific rendering to grid
// keys.rs     task specific key handling
// actions.rs  task specific action definitions if needed
// search.rs   task specific search items
// presets/    per distro data
//   ubuntu.rs
pub trait Task: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn category(&self) -> Category;
    fn supports_distro(&self, distro: &crate::core::distro::DistroId) -> bool;

    fn handle_key(&mut self, key: crate::core::key::Key) -> Vec<crate::core::action::Action>;
    fn render(&self, area: crate::core::rect::Rect, grid: &mut crate::core::grid::Grid);
    fn searchable_items(&self) -> Vec<crate::core::search::SearchItem> {
        Vec::new()
    }
}

// categories shown in the sidebar
// 1 package management 2 system management 3 networking and security
// 4 development toolchain 5 performance and gaming 6 environment
// categories 3 5 6 are empty for now they open but show nothing
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Category {
    PackageManagement,
    SystemManagement,
    NetworkingSecurity,
    DevelopmentToolchain,
    PerformanceGaming,
    Environment,
}

impl Category {
    pub fn display_name(self) -> &'static str {
        match self {
            Category::PackageManagement => "Package Management",
            Category::SystemManagement => "System Management",
            Category::NetworkingSecurity => "Networking and Security",
            Category::DevelopmentToolchain => "Development Toolchain",
            Category::PerformanceGaming => "Performance and Gaming",
            Category::Environment => "Environment",
        }
    }

    // returns true if this category has tasks
    // categories 3 5 6 are empty for now
    pub fn has_tasks(self) -> bool {
        matches!(
            self,
            Category::PackageManagement
                | Category::SystemManagement
                | Category::DevelopmentToolchain
        )
    }
}
