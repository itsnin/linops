// top level input dispatcher
// routes every key through three layers before it reaches a task
//
// layer 1 hard interrupts
// ctrl c is checked first in safe modes it quits the app
// in unsafe modes confirm and task ctrl c is reinterpreted as cancel operation
// this prevents accidental app exit during a destructive flow
//
// layer 2 shared structural keys
// esc up down are valid across all modes with the same semantic
// esc always means go back one level
// up down always mean move selection or scroll up down
// these are checked before the per mode handler so the user does not
// have to relearn structural navigation depending on which panel has focus
//
// layer 3 per mode handler
// each mode has its own handler that owns everything else
// only one handler ever sees a given keystroke
// no key means two different things in the same mode

pub fn dispatch(
    state: &mut crate::core::state::AppState,
    key: crate::core::key::Key,
) -> Vec<crate::core::action::Action> {
    // layer 1 hard interrupts
    if key == crate::core::key::Key::CtrlC {
        if state.mode.is_safe() {
            state.running = false;
            return Vec::new();
        }
        // unsafe mode ctrl c means cancel not quit
        return crate::core::handlers::cancel_current(state);
    }

    // layer 2 shared structural keys
    if key == crate::core::key::Key::Esc {
        return crate::core::handlers::go_back(state);
    }

    if key == crate::core::key::Key::Up {
        return crate::core::handlers::move_up(state);
    }

    if key == crate::core::key::Key::Down {
        return crate::core::handlers::move_down(state);
    }

    // layer 3 per mode handler
    match state.mode {
        crate::core::mode::Mode::Normal => crate::core::handlers::normal::handle(state, key),
        crate::core::mode::Mode::Search => crate::core::handlers::search::handle(state, key),
        crate::core::mode::Mode::Confirm => crate::core::handlers::confirm::handle(state, key),
        crate::core::mode::Mode::Help => crate::core::handlers::help::handle(state, key),
        crate::core::mode::Mode::Task => crate::core::handlers::task::handle(state, key),
    }
}
