// reads crossterm key events and maps them to core key
// this is the only place crossterm event types appear
// the rest of the code only sees core key
//
// shift variant policy
// shift tab is a distinct action previous task
// all other shift arrows are unhandled no op
// capital letters in search mode are literal uppercase text not commands
// capital letters in all other modes are unhandled
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

// polls for a key event with a 100ms timeout
// returns none if no event within the timeout
// returns some key if a recognized key was pressed
// unrecognized keys are skipped and we poll again
pub fn read_key() -> anyhow::Result<Option<linops_core::key::Key>> {
    if !event::poll(Duration::from_millis(100))? {
        return Ok(None);
    }

    match event::read()? {
        Event::Key(ev) => Ok(map_key(ev)),
        Event::Resize(width, height) => {
            // resize events are handled by the runner via the Resized message
            // we return none here and let the runner pick up the new size
            // on the next render cycle
            // this is a simplification the runner could also handle it directly
            let _ = (width, height);
            Ok(None)
        }
        _ => Ok(None),
    }
}

// maps a crossterm key event to our key enum
// returns none for keys we dont handle
fn map_key(ev: KeyEvent) -> Option<linops_core::key::Key> {
    match (ev.code, ev.modifiers) {
        (KeyCode::Tab, KeyModifiers::SHIFT) => Some(linops_core::key::Key::ShiftTab),
        (KeyCode::Tab, _) => Some(linops_core::key::Key::Tab),
        (KeyCode::Up, _) => Some(linops_core::key::Key::Up),
        (KeyCode::Down, _) => Some(linops_core::key::Key::Down),
        (KeyCode::Left, _) => Some(linops_core::key::Key::Left),
        (KeyCode::Right, _) => Some(linops_core::key::Key::Right),
        (KeyCode::Enter, _) => Some(linops_core::key::Key::Enter),
        (KeyCode::Esc, _) => Some(linops_core::key::Key::Esc),
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(linops_core::key::Key::CtrlC),
        (KeyCode::Char('/'), _) => Some(linops_core::key::Key::Slash),
        (KeyCode::Char('?'), KeyModifiers::SHIFT) => Some(linops_core::key::Key::Question),
        (KeyCode::Backspace, _) => Some(linops_core::key::Key::Backspace),
        (KeyCode::Char('y'), _) => Some(linops_core::key::Key::LowerY),
        (KeyCode::Char('n'), _) => Some(linops_core::key::Key::LowerN),
        (KeyCode::Char(ch), _) if ch.is_ascii() => Some(linops_core::key::Key::Char(ch)),
        _ => None,
    }
}
