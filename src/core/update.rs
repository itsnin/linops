// elm architecture update function
// takes state plus message returns new state plus actions
//
// for key pressed messages it calls the dispatcher
// for other messages task progress distro detected resize it updates state directly
pub fn update(
    mut state: crate::core::state::AppState,
    msg: crate::core::msg::Msg,
) -> (
    crate::core::state::AppState,
    Vec<crate::core::action::Action>,
) {
    match msg {
        crate::core::msg::Msg::KeyPressed(key) => {
            let actions = crate::core::dispatch::dispatch(&mut state, key);
            (state, actions)
        }
        crate::core::msg::Msg::DistroDetected(distro) => {
            state.distro = Some(distro);
            (state, Vec::new())
        }
        crate::core::msg::Msg::TaskStarted(name) => {
            state.current_task = Some(name);
            state.task_progress = 0;
            state.mode = crate::core::mode::Mode::Task;
            (state, Vec::new())
        }
        crate::core::msg::Msg::TaskProgress(_name, progress) => {
            state.task_progress = progress;
            (state, Vec::new())
        }
        crate::core::msg::Msg::TaskFinished(_name, success) => {
            state.current_task = None;
            state.task_progress = 0;
            state.mode = crate::core::mode::Mode::Normal;
            if !success {
                state.log_lines.push("task failed".to_string());
            }
            (state, Vec::new())
        }
        crate::core::msg::Msg::TaskLog(_name, line) => {
            state.log_lines.push(line);
            (state, Vec::new())
        }
        crate::core::msg::Msg::Resized(width, height) => {
            state.width = width;
            state.height = height;
            (state, Vec::new())
        }
    }
}
