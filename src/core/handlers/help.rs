// help mode handler
// read only overlay minimal key surface
// up down scroll handled by the dispatcher
// only question key toggles help off
pub fn handle(
    state: &mut crate::core::state::AppState,
    key: crate::core::key::Key,
) -> Vec<crate::core::action::Action> {
    match key {
        crate::core::key::Key::Question => {
            // toggle help off return to normal
            state.mode = crate::core::mode::Mode::Normal;
            Vec::new()
        }
        // all other keys are silently ignored
        _ => Vec::new(),
    }
}
