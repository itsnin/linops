// normal mode handler
// tab switches categories in the sidebar
// left right switches between sibling tasks within the current category
// up down is handled by the dispatcher before reaching here it scrolls
// the item list inside whichever task is currently active
// enter activates the selected item inside the active task
pub fn handle(
    state: &mut crate::state::AppState,
    key: crate::key::Key,
) -> Vec<crate::action::Action> {
    match key {
        crate::key::Key::Tab => {
            state.next_category();
            Vec::new()
        }
        crate::key::Key::ShiftTab => {
            state.prev_category();
            Vec::new()
        }
        crate::key::Key::Left => {
            state.prev_task();
            Vec::new()
        }
        crate::key::Key::Right => {
            state.next_task();
            Vec::new()
        }
        crate::key::Key::Enter => {
            let actions = state
                .active_task_mut()
                .map_or(Vec::new(), |t| t.handle_key(key));
            if actions
                .iter()
                .any(|a| !matches!(a, crate::action::Action::None))
            {
                state.mode = crate::mode::Mode::Confirm;
                state.pending_actions = actions;
                Vec::new()
            } else {
                actions
            }
        }
        crate::key::Key::Slash => {
            state.enter_search();
            Vec::new()
        }
        crate::key::Key::Question => {
            state.mode = crate::mode::Mode::Help;
            Vec::new()
        }
        _ => Vec::new(),
    }
}
