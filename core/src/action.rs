// actions are what tasks return when the user interacts
// every action runs through bash never through a rust reimplementation
// of apt dpkg systemctl etc a task embeds a verified bash script at
// compile time with include_str! and hands the source text here
//
// bash specifically not a portable posix sh every real linux install
// has bash available even when it is not the login shell so there is
// no multi shell compatibility burden here just always invoke bash
//
// the tui itself never runs as root needs_root true means the runner
// prefixes the call with sudo bash instead of plain bash see
// tui/runner.rs execute_action
//
// this replaces a typed per operation enum (install remove hold etc)
// on purpose a bash one liner is faster to write test and port from a
// verified script than a new rust match arm and adapter method per
// operation the goal is reaching a working task quickly not rust
// modeling every apt verb
#[derive(Clone, Debug)]
pub enum Action {
    // run a bash script name is a short label for logs not a path
    // the linops binary ships as a single file with no script
    // directory alongside it see scripts/start.sh so script must be
    // the full source text embedded at compile time not a path on disk
    RunScript {
        name: String,
        script: String,
        needs_root: bool,
    },
    // no action used when a key press does not produce an action
    None,
}
