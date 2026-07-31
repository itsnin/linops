// input mode controls which keys do what
// only one mode is active at a time
// safe modes allow hard interrupts like ctrl c to quit the app
// unsafe modes reinterpret ctrl c as cancel operation instead
// this prevents accidental app exit during a destructive flow
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Normal,
    Search,
    Confirm,
    Help,
    Task,
}

impl Mode {
    // returns true if this mode allows hard interrupts like ctrl c to quit
    // unsafe modes confirm and task reinterpret ctrl c as cancel instead
    pub fn is_safe(self) -> bool {
        matches!(self, Mode::Normal | Mode::Search | Mode::Help)
    }
}
