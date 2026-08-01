// terminal setup and teardown
// enter and leave are next to each other for easy review
// enter puts the terminal in raw mode and switches to the alternate screen
// raw mode so keystrokes reach us without line buffering
// alt screen so we dont clobber the users scrollback on exit
//
// also installs a panic hook that restores the terminal before printing
// this prevents the common tui bug where a panic leaves the terminal in raw mode
// and the panic message is unreadable garbage
use crossterm::terminal;
use std::sync::Once;

static PANIC_HOOK_INSTALLED: Once = Once::new();

pub fn enter() -> anyhow::Result<()> {
    install_panic_hook();
    terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), terminal::EnterAlternateScreen)?;
    Ok(())
}

pub fn leave() -> anyhow::Result<()> {
    crossterm::execute!(std::io::stdout(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// if the app panics we must restore the terminal first
// otherwise the user sees garbled output and has to run reset
// the hook calls leave then prints the panic to stderr
fn install_panic_hook() {
    PANIC_HOOK_INSTALLED.call_once(|| {
        let original = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = leave();
            original(info);
        }));
    });
}
