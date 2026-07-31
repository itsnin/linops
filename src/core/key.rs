// keys are the only inputs the app responds to
// no mouse no function keys no vim bindings
//
// normal mode uses tab shift tab arrows enter esc ctrl c slash question
// search mode also accepts backspace and printable chars for text input
// confirm mode also accepts y n for yes no
//
// in search mode printable chars are literal text not commands
// this is the critical rule of text input mode
// structural navigation letters are fully inert as commands in search mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Key {
    Tab,
    ShiftTab,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    CtrlC,
    // enters search mode only valid in normal mode
    Slash,
    // enters help overlay only valid in normal mode
    Question,
    // delete last char in search query only valid in search mode
    Backspace,
    // literal character input only valid in search mode
    Char(char),
    // yes shortcut only valid in confirm mode
    LowerY,
    // no shortcut only valid in confirm mode
    LowerN,
}
