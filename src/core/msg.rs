// messages are the only way the state changes
// the tui converts crossterm key events into keypressed messages
// background tasks send taskprogress messages
// the update function takes state plus a message and returns new state plus actions
//
// this is the elm architecture
// state plus messages plus update plus view
#[derive(Clone, Debug)]
pub enum Msg {
    KeyPressed(crate::core::key::Key),
    DistroDetected(crate::core::distro::DistroId),
    TaskStarted(String),
    TaskProgress(String, u8),
    TaskFinished(String, bool),
    TaskLog(String, String),
    // sent when the terminal is resized
    // the view function re layouts based on new size
    Resized(u16, u16),
}
