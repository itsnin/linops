// help mode handler
// read only overlay minimal key surface
// up down scroll handled by the dispatcher
// only question key toggles help off
pub fn handle(
    state: &mut crate::state::AppState,
    key: crate::key::Key,
) -> Vec<crate::action::Action> {
    match key {
        crate::key::Key::Question => {
            // toggle help off return to normal
            state.mode = crate::mode::Mode::Normal;
            Vec::new()
        }
        // all other keys are silently ignored
        _ => Vec::new(),
    }
}
