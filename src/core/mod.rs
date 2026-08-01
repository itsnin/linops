// core has zero ui dependencies and zero task dependencies
// it defines traits and types that tasks implement
// tui only no gui
//
// core philosophy
// the util prioritizes less bugs and just works
// no config file required defaults are baked in
// auto detect everything distro terminal size color support
// graceful degradation unknown distro show available tasks small terminal show message
// no setup steps curl bash run it works
//
// input philosophy
// mode scoped input not global input
// one top level dispatcher routes every key through three layers
// 1 hard interrupts ctrl c only in safe modes never during destructive ops
// 2 shared structural keys esc up down valid across all modes
// 3 per mode handler owns everything else
// only one handler ever sees a given keystroke no priority fights
//
// task philosophy
// each task is a fully self contained rust module
// a task owns its state its ui its keys its actions its presets its search items
// task a code never touches task b code
// the core never imports from tasks
// tasks import core traits only
// the registry in tasks/mod.rs is the only place that knows about all tasks
pub mod action;
pub mod charset;
pub mod color;
pub mod dispatch;
pub mod distro;
pub mod grid;
pub mod handlers;
pub mod key;
pub mod mode;
pub mod msg;
pub mod rect;
pub mod search;
pub mod state;
pub mod task;
pub mod theme;
pub mod update;
pub mod view;
