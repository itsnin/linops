// search mode handler
// critical rule printable chars are literal text not commands
// structural navigation letters are fully inert as commands here
// only backspace enter left right are active as commands
//
// left right in search mode move the cursor within the query text
// this is different from normal mode where left right are structural navigation
// this cross mode difference is documented in PLAN.md
pub fn handle(
    state: &mut crate::state::AppState,
    key: crate::key::Key,
) -> Vec<crate::action::Action> {
    match key {
        crate::key::Key::Char(ch) => {
            state.search_query.push(ch);
            state.update_search_results();
            Vec::new()
        }
        crate::key::Key::Backspace => {
            state.search_query.pop();
            state.update_search_results();
            Vec::new()
        }
        crate::key::Key::Enter => {
            // submit jumps to the selected result
            // the query is applied meaning the search context is preserved
            // not discarded this is the deliberate default
            state.jump_to_search_result();
            Vec::new()
        }
        // tab is inert in search mode to avoid losing typed text
        // all other keys are silently ignored
        _ => Vec::new(),
    }
}
