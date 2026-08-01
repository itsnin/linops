// normal mode handler
// tab switches categories in the sidebar
// up down navigates tasks within the current category
// enter activates the selected task
pub fn handle(
    state: &mut crate::core::state::AppState,
    key: crate::core::key::Key,
) -> Vec<crate::core::action::Action> {
    match key {
        crate::core::key::Key::Tab => {
            state.next_category();
            Vec::new()
        }
        crate::core::key::Key::ShiftTab => {
            state.prev_category();
            Vec::new()
        }
        crate::core::key::Key::Left | crate::core::key::Key::Right => state
            .active_task_mut()
            .map_or(Vec::new(), |t| t.handle_key(key)),
        crate::core::key::Key::Enter => {
            let actions = state
                .active_task_mut()
                .map_or(Vec::new(), |t| t.handle_key(key));
            if actions
                .iter()
                .any(|a| !matches!(a, crate::core::action::Action::None))
            {
                state.mode = crate::core::mode::Mode::Confirm;
                state.pending_actions = actions;
                Vec::new()
            } else {
                actions
            }
        }
        crate::core::key::Key::Slash => {
            state.enter_search();
            Vec::new()
        }
        crate::core::key::Key::Question => {
            state.mode = crate::core::mode::Mode::Help;
            Vec::new()
        }
        _ => Vec::new(),
    }
}
