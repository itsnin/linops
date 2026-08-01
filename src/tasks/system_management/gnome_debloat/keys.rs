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
            match state.selected_index {
                0 => {
                    state.remove_gnome_apps = !state.remove_gnome_apps;
                }
                1 => {
                    state.remove_ptyxis = !state.remove_ptyxis;
                }
                _ => {}
            }
            Vec::new()
        }
        _ => Vec::new(),
    }
}
