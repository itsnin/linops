// tui binary depends on ratatui and crossterm
// this is the only crate in the workspace allowed to depend on them
// linops_core (../core) has zero dependency on either which is
// what keeps the render layer swappable later see
// ../../scripts/LLM.md for the full reasoning
//
// runner::run currently still calls into the old single crate
// linops::tui path this needs to change to call linops_core once
// that crate exposes its new tabs discovery api see LLM.md status
// section this file is mid rewrite not a finished entry point yet
mod input;
mod render;
mod runner;
mod terminal;

fn main() -> anyhow::Result<()> {
    runner::run()
}
