// confirm mode handler
// blocking dialog for destructive action confirmation
// supports yes no via enter and letter shortcuts y n
// left right toggle between yes and no
//
// ctrl c is handled by the dispatcher as cancel not quit
// this is the cross mode difference documented in PLAN.md
// ctrl c quits in normal search help but cancels in confirm task
pub fn handle(
    state: &mut crate::core::state::AppState,
    key: crate::core::key::Key,
) -> Vec<crate::core::action::Action> {
    match key {
        crate::core::key::Key::Enter => {
            // confirm yes execute the pending actions
            let actions = std::mem::take(&mut state.pending_actions);
            state.mode = crate::core::mode::Mode::Task;
            actions
        }
        crate::core::key::Key::LowerY => {
            let actions = std::mem::take(&mut state.pending_actions);
            state.mode = crate::core::mode::Mode::Task;
            actions
        }
        crate::core::key::Key::LowerN => {
            // decline clear pending actions return to normal
            state.pending_actions.clear();
            state.mode = crate::core::mode::Mode::Normal;
            Vec::new()
        }
        // all other keys are silently ignored
        _ => Vec::new(),
    }
}
