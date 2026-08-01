#[derive(Default)]
pub struct State {
    pub remove_kdump: bool,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            remove_kdump: true,
            selected_index: 0,
        }
    }
}
