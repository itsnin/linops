// main event loop
// owns the ratatui terminal and drives the elm architecture cycle
//
// loop
//   poll key events from crossterm with 100ms timeout
//   if key event convert to msg call core update
//   if no key event check terminal size for resize
//   execute returned actions each is an embedded bash script
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
    crate::terminal::enter()?;

    // defer leave so it runs even if we return early
    let result = run_loop();

    // always restore terminal
    let _ = crate::terminal::leave();

    result
}

fn run_loop() -> anyhow::Result<()> {
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

    // create app state with builtin tasks
    let tasks = crate::tasks::builtin_tasks();
    let mut state = linops_core::state::AppState::new(tasks);

    // detect distro for display and future per distro script variants
    // execution itself never routes through a rust adapter see
    // core/action.rs every mutation runs as an embedded bash script
    let distro = linops_core::distro::detect::detect();
    state.distro = Some(distro);

    // get terminal size for initial render
    let (width, height) = crossterm::terminal::size()?;
    state.width = width;
    state.height = height;

    let theme = linops_core::theme::Theme::default_theme();

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
        match crate::input::read_key()? {
            Some(key) => {
                let msg = linops_core::msg::Msg::KeyPressed(key);
                let (new_state, actions) = linops_core::update::update(state, msg);
                state = new_state;

                // execute returned actions each runs an embedded bash
                // script errors are recorded as log lines rather than
                // aborting the loop a failed script should not crash
                // the tui
                for action in actions {
                    execute_action(&mut state, action);
                }
            }
            None => {
                // no key event within timeout just continue to render
            }
        }

        // render
        let grid = linops_core::view::view(&state, &theme);

        terminal.draw(|frame| {
            let area = frame.area();
            // render directly into the frame buffer
            crate::render::render_grid(&grid, frame.buffer_mut(), area);
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

// runs one action and records a short result line
// a failed script is logged not fatal the user can retry or investigate
// the full script source never lands in the log only the short name
fn execute_action(state: &mut linops_core::state::AppState, action: linops_core::action::Action) {
    let linops_core::action::Action::RunScript {
        name,
        script,
        needs_root,
    } = &action
    else {
        return;
    };

    match run_script(script, *needs_root) {
        Ok(()) => state.log_lines.push(format!("done: {name}")),
        Err(e) => state.log_lines.push(format!("failed: {name} ({e})")),
    }
}

// pipes the embedded script text into bash on stdin
// bash specifically not sh every real linux install has bash even when
// it is not the login shell so there is no portability concern here
// sudo bash rather than sudo -u since needs_root means the whole
// script runs as root the same way debloat.sh re execs itself as root
// piping via stdin avoids writing and cleaning up a temp file on disk
fn run_script(script: &str, needs_root: bool) -> anyhow::Result<()> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = if needs_root {
        Command::new("sudo")
            .arg("bash")
            .stdin(Stdio::piped())
            .spawn()?
    } else {
        Command::new("bash").stdin(Stdio::piped()).spawn()?
    };

    // stdin is guaranteed present we just requested Stdio::piped above
    let mut stdin = child.stdin.take().expect("piped stdin is always present");
    stdin.write_all(script.as_bytes())?;
    drop(stdin);

    let status = child.wait()?;
    if !status.success() {
        anyhow::bail!("script exited with {status}");
    }
    Ok(())
}
