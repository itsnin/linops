// system_update state
// holds whether update and upgrade are enabled
#[derive(Default)]
pub struct State {
    pub do_update: bool,
    pub do_upgrade: bool,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            do_update: true,
            do_upgrade: true,
            selected_index: 0,
        }
    }
}
