#[derive(Default)]
pub struct State {
    pub remove_gnome_apps: bool,
    pub remove_ptyxis: bool,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            remove_gnome_apps: true,
            remove_ptyxis: true,
            selected_index: 0,
        }
    }
}
