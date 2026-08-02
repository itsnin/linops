# architecture

```
                    workspace (see root Cargo.toml)
                                |
              +-----------------+-----------------+
              |                                   |
        core crate (linops_core)            tui crate (linops)
        zero ratatui crossterm dep          the shipped binary
        distro detect grid color            depends on linops_core
        charset key rect theme mode         ratatui crossterm here only
        search action state view            terminal input render runner
        dispatch handlers
              |
        core/tabs/ data tree
        tabs.toml + tab_data.toml
        per category one embedded
        bash script per task
              |
        Grid<Cell> the shared render target
              |
        tui renderer converts grid to a
        ratatui Buffer this is the one
        place that would change if
        ratatui were ever swapped out
```

a third `xtask` crate exists for project local dev tooling (docgen
style, matching linutil's own xtask) it is not part of the shipped
binary and default-members in the root Cargo.toml excludes it from
a plain cargo build

## principles

1. `linops_core` has zero dependency on ratatui or crossterm not
   even listed in its Cargo.toml this is a real compile time
   boundary not just a convention see scripts/LLM.md
2. every task is exactly one bash script embedded at build time no
   rust struct per task no per task state ui keys files
3. the tui process never runs as root only the one script the user
   chooses to run gets escalated via sudo bash for that invocation
4. 16 colors cp437 charset only because that is what a bare tty1
   console can render
5. keyboard only no mouse
6. elm style architecture state msg update view still applies to
   the tui shell itself even though tasks are no longer elm style
   rust modules
7. just works philosophy no config required auto detect everything
8. mode scoped input one dispatcher three layers see core/src/dispatch.rs
9. tui only no gui a separate macos/windows tool would be a
   different project entirely not a mode of this one

this file describes the target shape some of it is mid rewrite as
of this writing see scripts/LLM.md's status section for what is
actually finished versus still aspirational
