// per mode key handlers
// shared keys esc up down ctrl c are handled by the dispatcher before reaching here
pub mod confirm;
pub mod help;
pub mod normal;
pub mod search;
pub mod task;

// shared handler for esc go back one level
// never quits the app just returns to the previous mode
pub fn go_back(state: &mut crate::state::AppState) -> Vec<crate::action::Action> {
    match state.mode {
        crate::mode::Mode::Search => state.exit_search(),
        crate::mode::Mode::Confirm => {
            state.pending_actions.clear();
            state.mode = crate::mode::Mode::Normal;
        }
        crate::mode::Mode::Help => {
            state.mode = crate::mode::Mode::Normal;
        }
        crate::mode::Mode::Task => {
            // cancel the running task
            state.current_task = None;
            state.task_progress = 0;
            state.mode = crate::mode::Mode::Normal;
        }
        crate::mode::Mode::Normal => {
            // in normal mode esc does nothing
            // it does not quit the app only ctrl c does that
        }
    }
    Vec::new()
}

// shared handler for up arrow
// scrolls the item list inside the currently active task
// this mirrors linutil focus list where up down move the list selection
// and left right change which list has focus (see handlers/normal.rs)
pub fn move_up(state: &mut crate::state::AppState) -> Vec<crate::action::Action> {
    match state.mode {
        crate::mode::Mode::Normal => {
            return state
                .active_task_mut()
                .map_or(Vec::new(), |t| t.handle_key(crate::key::Key::Up));
        }
        crate::mode::Mode::Search if state.search_selected > 0 => {
            state.search_selected -= 1;
        }
        _ => {}
    }
    Vec::new()
}

// shared handler for down arrow
// scrolls the item list inside the currently active task
pub fn move_down(state: &mut crate::state::AppState) -> Vec<crate::action::Action> {
    match state.mode {
        crate::mode::Mode::Normal => {
            return state
                .active_task_mut()
                .map_or(Vec::new(), |t| t.handle_key(crate::key::Key::Down));
        }
        crate::mode::Mode::Search
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
    state: &mut crate::state::AppState,
) -> Vec<crate::action::Action> {
    match state.mode {
        crate::mode::Mode::Confirm => {
            state.pending_actions.clear();
            state.mode = crate::mode::Mode::Normal;
        }
        crate::mode::Mode::Task => {
            state.current_task = None;
            state.task_progress = 0;
            state.mode = crate::mode::Mode::Normal;
        }
        _ => {}
    }
    Vec::new()
}
