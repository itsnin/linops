# linops implementation plan

> status phase 3 tui shell implemented
> version 2026.08.01 calendar date based
> tui only no gui

## categories

1. package management - active (software_install)
2. system management - active (snap_debloat apt_pins system_update apt_cleanup kdump_removal upstream_gnome gnome_debloat)
3. networking and security - empty
4. development toolchain - active (dev_toolchain)
5. performance and gaming - empty
6. environment - empty

## tasks

| category | task | what it does |
|---|---|---|
| package management | software_install | install htop wget fonts-noto gnome-boxes micro chrome extension-manager |
| system management | snap_debloat | remove snapd pin it |
| system management | apt_pins | write apt pin file |
| system management | system_update | apt update + upgrade |
| system management | apt_cleanup | mark manual drop gnome-core autoremove |
| system management | kdump_removal | remove kdump-tools free 512mb |
| system management | upstream_gnome | install gnome-core gdm ghostty network-manager xdg-user-dirs |
| system management | gnome_debloat | remove 23 gnome apps ptyxis |
| development toolchain | dev_toolchain | python c c++ rust java nodejs dbs web tooling |

## phased plan

### phase 1 scaffolding done
- 6 categories 9 tasks all scaffolded
- core framework skeleton
- tui shell skeleton
- helper binary skeleton

### phase 2 core framework done
- grid write_str box_border fill implemented
- color ansi fg bg rgb implemented
- charset is_safe sanitize implemented
- fuzzy search subsequence matching with scoring implemented
- distro detect parse /etc/os-release implemented
- ubuntu adapter all methods implemented
- task trait and category enum implemented
- state with search index and results implemented
- update elm architecture implemented
- view with topbar sidebar main statusbar help search implemented
- dispatch 3 layer dispatcher implemented
- handlers all 5 mode handlers implemented
- tests for grid charset search distro

### phase 3 tui shell done
- terminal enter leave with panic hook implemented
- input read_key using crossterm poll read implemented
- render grid to ratatui buffer conversion implemented
- runner main event loop with sigterm handling implemented
- main rs launches the tui
- task stubs made non-panicking render placeholder message
- cargo run opens a working tui with sidebar categories and tasks

### phase 4 implement tasks next
- port each ubuntu-debloat script section into its task
- implement task state ui keys search presets
- test end to end on ubuntu 2604 server vm

### phase 5 distribution
- implement install sh and release pipeline
- tag v2026.08.01
