#[derive(Default)]
pub struct State {
    pub install_gnome_core: bool,
    pub install_gdm: bool,
    pub install_ghostty: bool,
    pub install_network_manager: bool,
    pub install_xdg_dirs: bool,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            install_gnome_core: true,
            install_gdm: true,
            install_ghostty: true,
            install_network_manager: true,
            install_xdg_dirs: true,
            selected_index: 0,
        }
    }

    pub fn item_count() -> usize {
        5
    }

    pub fn is_checked(&self, i: usize) -> bool {
        match i {
            0 => self.install_gnome_core,
            1 => self.install_gdm,
            2 => self.install_ghostty,
            3 => self.install_network_manager,
            4 => self.install_xdg_dirs,
            _ => false,
        }
    }

    pub fn toggle(&mut self, i: usize) {
        match i {
            0 => self.install_gnome_core = !self.install_gnome_core,
            1 => self.install_gdm = !self.install_gdm,
            2 => self.install_ghostty = !self.install_ghostty,
            3 => self.install_network_manager = !self.install_network_manager,
            4 => self.install_xdg_dirs = !self.install_xdg_dirs,
            _ => {}
        }
    }

    pub fn names() -> &'static [(&'static str, &'static str)] {
        &[
            (
                "gnome-core",
                "vanilla gnome shell session control center no recommends",
            ),
            ("gdm3", "gnome display manager enable graphical target"),
            ("ghostty", "gpu accelerated terminal register as default"),
            (
                "network-manager",
                "replace netplan networkd with networkmanager",
            ),
            (
                "xdg-user-dirs-gtk",
                "create desktop documents downloads music pictures videos on first login",
            ),
        ]
    }
}
