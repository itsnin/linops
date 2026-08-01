#[derive(Default)]
pub struct State {
    pub write_pins: bool,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            write_pins: true,
            selected_index: 0,
        }
    }
}
