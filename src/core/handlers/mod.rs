// per mode key handlers
// shared keys esc up down ctrl c are handled by the dispatcher before reaching here
pub mod confirm;
pub mod help;
pub mod normal;
pub mod search;
pub mod task;

// shared handler for esc go back one level
// never quits the app just returns to the previous mode
pub fn go_back(state: &mut crate::core::state::AppState) -> Vec<crate::core::action::Action> {
    match state.mode {
        crate::core::mode::Mode::Search => state.exit_search(),
        crate::core::mode::Mode::Confirm => {
            state.pending_actions.clear();
            state.mode = crate::core::mode::Mode::Normal;
        }
        crate::core::mode::Mode::Help => {
            state.mode = crate::core::mode::Mode::Normal;
        }
        crate::core::mode::Mode::Task => {
            // cancel the running task
            state.current_task = None;
            state.task_progress = 0;
            state.mode = crate::core::mode::Mode::Normal;
        }
        crate::core::mode::Mode::Normal => {
            // in normal mode esc does nothing
            // it does not quit the app only ctrl c does that
        }
    }
    Vec::new()
}

// shared handler for up arrow
// navigates tasks within the current category
pub fn move_up(state: &mut crate::core::state::AppState) -> Vec<crate::core::action::Action> {
    match state.mode {
        crate::core::mode::Mode::Normal => {
            state.prev_task();
        }
        crate::core::mode::Mode::Search if state.search_selected > 0 => {
            state.search_selected -= 1;
        }
        _ => {}
    }
    Vec::new()
}

// shared handler for down arrow
// navigates tasks within the current category
pub fn move_down(state: &mut crate::core::state::AppState) -> Vec<crate::core::action::Action> {
    match state.mode {
        crate::core::mode::Mode::Normal => {
            state.next_task();
        }
        crate::core::mode::Mode::Search
            if state.search_selected + 1 < state.search_results.len() =>
        {
            state.search_selected += 1;
        }
        _ => {}
    }
    Vec::new()
}

// shared handler for ctrl c in unsafe modes
// cancel the current operation and return to normal
pub fn cancel_current(
    state: &mut crate::core::state::AppState,
) -> Vec<crate::core::action::Action> {
    match state.mode {
        crate::core::mode::Mode::Confirm => {
            state.pending_actions.clear();
            state.mode = crate::core::mode::Mode::Normal;
        }
        crate::core::mode::Mode::Task => {
            state.current_task = None;
            state.task_progress = 0;
            state.mode = crate::core::mode::Mode::Normal;
        }
        _ => {}
    }
    Vec::new()
}
