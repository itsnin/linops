// the distro adapter trait combines package manager and service manager
// each distro implements this to translate abstract operations into real commands
// tasks call these methods and never branch on distro themselves
pub trait DistroAdapter: Send + Sync {
    fn id(&self) -> crate::core::distro::DistroId;
    fn display_name(&self) -> &str;

    fn install(&self, packages: &[String]) -> anyhow::Result<()>;
    fn remove(&self, packages: &[String]) -> anyhow::Result<()>;
    fn purge(&self, packages: &[String]) -> anyhow::Result<()>;
    fn mark_manual(&self, packages: &[String]) -> anyhow::Result<()>;
    fn autoremove_purge(&self) -> anyhow::Result<()>;
    fn is_installed(&self, package: &str) -> bool;

    fn enable_service(&self, service: &str) -> anyhow::Result<()>;
    fn disable_service(&self, service: &str) -> anyhow::Result<()>;
    fn set_default_target(&self, target: &str) -> anyhow::Result<()>;

    // some distros have a pin mechanism to block packages from reinstalling
    // ubuntu uses apt preferences other distros may differ
    fn write_pin_file(&self, path: &str, packages: &[String]) -> anyhow::Result<()>;
}

// returns the right adapter for the detected distro
// only ubuntu for now
pub fn pick(distro: crate::core::distro::DistroId) -> Box<dyn DistroAdapter> {
    match distro {
        crate::core::distro::DistroId::Ubuntu => {
            Box::new(crate::core::distro::adapters::ubuntu::UbuntuAdapter::new())
        }
        _ => Box::new(crate::core::distro::adapters::ubuntu::UbuntuAdapter::new()),
    }
}
