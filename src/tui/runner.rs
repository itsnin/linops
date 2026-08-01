// main event loop
// owns the ratatui terminal and drives the elm architecture cycle
//
// loop
//   poll key events from crossterm with 100ms timeout
//   if key event convert to msg call core update
//   if no key event check terminal size for resize
//   execute returned actions for now just log them
//   call core view to get a grid
//   convert grid to ratatui buffer
//   render to terminal
//   repeat until state.running is false
//
// signal handling
// ctrl c in raw mode is delivered as a key event not a signal
// so the normal key handler catches it
// sigterm kill is handled by a background thread that sets a flag
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

pub fn run() -> anyhow::Result<()> {
    // install signal handler for sigterm
    // when sigterm arrives set the exit flag
    // the main loop checks this flag each iteration
    install_signal_handler();

    // enter terminal raw mode and alt screen
    crate::tui::terminal::enter()?;

    // defer leave so it runs even if we return early
    let result = run_loop();

    // always restore terminal
    let _ = crate::tui::terminal::leave();

    result
}

fn run_loop() -> anyhow::Result<()> {
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

    // create app state with builtin tasks
    let tasks = crate::tasks::builtin_tasks();
    let mut state = crate::core::state::AppState::new(tasks);

    // detect distro
    let distro = crate::core::distro::detect::detect();
    state.distro = Some(distro);

    // get terminal size for initial render
    let (width, height) = crossterm::terminal::size()?;
    state.width = width;
    state.height = height;

    let theme = crate::core::theme::Theme::default_theme();

    // main loop
    loop {
        // check exit flags
        if !state.running || SHOULD_EXIT.load(Ordering::Relaxed) {
            break;
        }

        // check for terminal resize
        let (new_w, new_h) = crossterm::terminal::size()?;
        if new_w != state.width || new_h != state.height {
            state.width = new_w;
            state.height = new_h;
            terminal.resize(ratatui::layout::Rect::new(0, 0, new_w, new_h))?;
        }

        // poll for key event with 100ms timeout
        match crate::tui::input::read_key()? {
            Some(key) => {
                let msg = crate::core::msg::Msg::KeyPressed(key);
                let (new_state, actions) = crate::core::update::update(state, msg);
                state = new_state;

                // execute returned actions
                // for now just log them
                // actual execution via distro adapter comes in phase 4
                for action in actions {
                    if !matches!(action, crate::core::action::Action::None) {
                        state.log_lines.push(format!("action: {:?}", action));
                    }
                }
            }
            None => {
                // no key event within timeout just continue to render
            }
        }

        // render
        let grid = crate::core::view::view(&state, &theme);

        terminal.draw(|frame| {
            let area = frame.area();
            // render directly into the frame buffer
            crate::tui::render::render_grid(&grid, frame.buffer_mut(), area);
        })?;
    }

    Ok(())
}

fn install_signal_handler() {
    thread::spawn(|| {
        // set up a simple sigterm handler
        // on unix sigterm is signal 15
        // we use a raw signal handler via libc
        // this is the simplest approach for a tui app
        //
        // alternatively we could use tokio::signal but that requires
        // a tokio runtime which we dont have in this synchronous loop
        //
        // the signal handler just sets an atomic flag
        // the main loop checks it each iteration
        #[cfg(unix)]
        unsafe {
            libc::signal(
                libc::SIGTERM,
                handle_sigterm as *const () as libc::sighandler_t,
            );
        }
    });
}

#[cfg(unix)]
extern "C" fn handle_sigterm(_sig: i32) {
    SHOULD_EXIT.store(true, Ordering::Relaxed);
}
