// task registry
pub mod dev_toolchain;
pub mod environment;
pub mod networking_security;
pub mod package_management;
pub mod performance_gaming;
pub mod system_management;

use crate::core::task::Task;

pub fn builtin_tasks() -> Vec<Box<dyn Task>> {
    vec![
        // package management
        Box::new(package_management::web_browser::WebBrowser::new()),
        Box::new(package_management::system_util::SystemUtil::new()),
        Box::new(package_management::fonts::Fonts::new()),
        Box::new(package_management::virtualization::Virtualization::new()),
        Box::new(package_management::editor::Editor::new()),
        Box::new(package_management::gnome_ext::GnomeExt::new()),
        // system management
        Box::new(system_management::snap_debloat::SnapDebloat::new()),
        Box::new(system_management::apt_pins::AptPins::new()),
        Box::new(system_management::system_update::SystemUpdate::new()),
        Box::new(system_management::apt_cleanup::AptCleanup::new()),
        Box::new(system_management::kdump_removal::KdumpRemoval::new()),
        Box::new(system_management::upstream_gnome::UpstreamGnome::new()),
        Box::new(system_management::gnome_debloat::GnomeDebloat::new()),
        // dev toolchain
        Box::new(dev_toolchain::dev_toolchain::DevToolchain::new()),
    ]
}
