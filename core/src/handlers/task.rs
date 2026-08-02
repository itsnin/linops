// task mode handler
// background task is running shows progress and live log
// only esc and ctrl c to cancel both handled by the dispatcher
// up down scroll the log handled by the dispatcher
// all other keys are silently ignored
//
// this is not a passthrough mode
// linops helper communicates via json over stdin stdout
// the user never types into the subprocess directly
// if a task needs interactive input it should use linops own confirm dialog
// not the subprocess prompt
pub fn handle(
    _state: &mut crate::state::AppState,
    _key: crate::key::Key,
) -> Vec<crate::action::Action> {
    // all keys are silently ignored in task mode
    // only esc and ctrl c work handled by dispatcher
    Vec::new()
}
