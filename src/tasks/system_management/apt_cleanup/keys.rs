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
            if state.selected_index < 2 {
                state.selected_index += 1;
            }
            Vec::new()
        }
        crate::core::key::Key::Enter => {
            match state.selected_index {
                0 => {
                    state.mark_manual = !state.mark_manual;
                }
                1 => {
                    state.drop_gnome_core = !state.drop_gnome_core;
                }
                2 => {
                    state.autoremove = !state.autoremove;
                }
                _ => {}
            }
            Vec::new()
        }
        _ => Vec::new(),
    }
}
