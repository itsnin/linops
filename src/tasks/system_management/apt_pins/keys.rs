pub fn handle(
    state: &mut super::state::State,
    key: crate::core::key::Key,
) -> Vec<crate::core::action::Action> {
    match key {
        crate::core::key::Key::Enter => {
            state.write_pins = !state.write_pins;
            Vec::new()
        }
        _ => Vec::new(),
    }
}
