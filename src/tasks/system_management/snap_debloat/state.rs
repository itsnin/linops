// snap_debloat state
#[derive(Default)]
pub struct State {
    pub remove_snapd: bool,
    pub pin_snapd: bool,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            remove_snapd: true,
            pin_snapd: true,
            selected_index: 0,
        }
    }
}
