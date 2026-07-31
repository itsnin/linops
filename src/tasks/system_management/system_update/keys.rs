// system_update key handling
// up down moves selection
// enter toggles the selected option or runs if both are toggled
pub fn handle(
    state: &mut super::state::State,
    key: crate::core::key::Key,
) -> Vec<crate::core::action::Action> {
    match key {
        crate::core::key::Key::Up => {
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
            Vec::new()
        }
        crate::core::key::Key::Down => {
            if state.selected_index < 1 {
                state.selected_index += 1;
            }
            Vec::new()
        }
        crate::core::key::Key::Enter => {
            // toggle the selected option
            match state.selected_index {
                0 => {
                    state.do_update = !state.do_update;
                }
                1 => {
                    state.do_upgrade = !state.do_upgrade;
                }
                _ => {}
            }
            Vec::new()
        }
        _ => Vec::new(),
    }
}
