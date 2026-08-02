// linops_core has zero dependency on ratatui or crossterm not even
// in Cargo.toml this crate boundary is what keeps the render layer
// swappable later see ../../scripts/LLM.md for the full reasoning
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
// task philosophy see scripts/LLM.md for the full binding rules
// every task is exactly one bash script embedded at build time not
// a rust struct there is no per task state to route keys into a
// task is discovered by walking the tabs directory tree not by a
// hand written registry this file's module list below and the
// modules themselves are mid rewrite to match this see LLM.md
// status section before assuming any of it is finished
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
