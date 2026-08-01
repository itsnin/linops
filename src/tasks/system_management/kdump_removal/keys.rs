pub fn handle(
    state: &mut super::state::State,
    key: crate::core::key::Key,
) -> Vec<crate::core::action::Action> {
    match key {
        crate::core::key::Key::Enter => {
            state.remove_kdump = !state.remove_kdump;
            Vec::new()
        }
        _ => Vec::new(),
    }
}
