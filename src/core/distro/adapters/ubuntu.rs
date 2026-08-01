// ubuntu adapter
// apt for packages systemd for services
// also covers derivatives like linux mint pop os etc
// because they share the same apt and systemd base
use std::process::Command;

#[derive(Default)]
pub struct UbuntuAdapter;

impl UbuntuAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl crate::core::distro::adapter::DistroAdapter for UbuntuAdapter {
    fn id(&self) -> crate::core::distro::DistroId {
        crate::core::distro::DistroId::Ubuntu
    }

    fn display_name(&self) -> &str {
        "Ubuntu"
    }

    fn install(&self, packages: &[String]) -> anyhow::Result<()> {
        let status = Command::new("apt-get")
            .args(["install", "-y"])
            .args(packages)
            .status()?;
        if !status.success() {
            anyhow::bail!("apt-get install failed");
        }
        Ok(())
    }

    fn remove(&self, packages: &[String]) -> anyhow::Result<()> {
        let status = Command::new("apt-get")
            .args(["remove", "-y"])
            .args(packages)
            .status()?;
        if !status.success() {
            anyhow::bail!("apt-get remove failed");
        }
        Ok(())
    }

    fn purge(&self, packages: &[String]) -> anyhow::Result<()> {
        let status = Command::new("apt-get")
            .args(["remove", "-y", "--purge"])
            .args(packages)
            .status()?;
        if !status.success() {
            anyhow::bail!("apt-get purge failed");
        }
        Ok(())
    }

    fn mark_manual(&self, packages: &[String]) -> anyhow::Result<()> {
        let status = Command::new("apt-mark")
            .args(["manual"])
            .args(packages)
            .status()?;
        if !status.success() {
            anyhow::bail!("apt-mark manual failed");
        }
        Ok(())
    }

    fn autoremove_purge(&self) -> anyhow::Result<()> {
        let status = Command::new("apt-get")
            .args(["autoremove", "-y", "--purge"])
            .status()?;
        if !status.success() {
            anyhow::bail!("apt-get autoremove purge failed");
        }
        Ok(())
    }

    fn is_installed(&self, package: &str) -> bool {
        Command::new("dpkg")
            .args(["-s", package])
            .output()
            .map(|o| {
                o.status.success()
                    && String::from_utf8_lossy(&o.stdout).contains("Status: install ok installed")
            })
            .unwrap_or(false)
    }

    fn enable_service(&self, service: &str) -> anyhow::Result<()> {
        let status = Command::new("systemctl")
            .args(["enable", service])
            .status()?;
        if !status.success() {
            anyhow::bail!("systemctl enable failed");
        }
        Ok(())
    }

    fn disable_service(&self, service: &str) -> anyhow::Result<()> {
        let status = Command::new("systemctl")
            .args(["disable", service])
            .status()?;
        if !status.success() {
            anyhow::bail!("systemctl disable failed");
        }
        Ok(())
    }

    fn set_default_target(&self, target: &str) -> anyhow::Result<()> {
        let status = Command::new("systemctl")
            .args(["set-default", target])
            .status()?;
        if !status.success() {
            anyhow::bail!("systemctl set-default failed");
        }
        Ok(())
    }

    fn write_pin_file(&self, path: &str, packages: &[String]) -> anyhow::Result<()> {
        let pkg_list = packages.join(" ");
        let content = format!(
            "# linops pin file\nPackage: {}\nPin: release *\nPin-Priority: -1\n",
            pkg_list
        );
        std::fs::write(path, content)?;
        Ok(())
    }
}
