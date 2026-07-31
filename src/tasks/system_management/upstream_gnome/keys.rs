pub fn handle(
    state: &mut super::state::State,
    key: crate::core::key::Key,
) -> Vec<crate::core::action::Action> {
    let max = super::state::State::item_count() - 1;
    match key {
        crate::core::key::Key::Up => {
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
            Vec::new()
        }
        crate::core::key::Key::Down => {
            if state.selected_index < max {
                state.selected_index += 1;
            }
            Vec::new()
        }
        crate::core::key::Key::Enter => {
            state.toggle(state.selected_index);
            Vec::new()
        }
        _ => Vec::new(),
    }
}
