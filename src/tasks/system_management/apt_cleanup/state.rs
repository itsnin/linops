#[derive(Default)]
pub struct State {
    pub mark_manual: bool,
    pub drop_gnome_core: bool,
    pub autoremove: bool,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            mark_manual: true,
            drop_gnome_core: true,
            autoremove: true,
            selected_index: 0,
        }
    }
}
