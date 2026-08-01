# linops

> status phase 5 verified tui launches and renders correctly
> version 2026.8.1

a modular linux system management tui

one binary no system dependencies runs on any linux distro from the kernel console to wayland terminals

## philosophy

linops prioritizes less bugs and just works
it gets out of your way and does what you want
without making you spend time configuring things

- no config file required defaults are baked in
- auto detect everything distro terminal size color support
- sensible defaults every toggle starts in a reasonable state
- graceful degradation unknown distro show available tasks small terminal show message
- no setup steps curl bash run it works

## run

```bash
curl -fsSL https://raw.githubusercontent.com/itsnin/linops/main/scripts/start.sh | sh
```

downloads linops to a temp file runs it then deletes it
no rust no cargo no system changes nothing is installed

always fetches the latest build from the continuous release

## categories and tasks

| category | task | what it does |
|---|---|---|
| package management | software_install | install htop wget fonts-noto gnome-boxes micro chrome |
| system management | snap_debloat | remove snapd and pin it |
| system management | apt_pins | write apt pin files |
| system management | system_update | apt update and upgrade |
| system management | apt_cleanup | mark manual drop metapackages autoremove |
| system management | kdump_removal | remove kdump-tools free 512mb |
| system management | upstream_gnome | install vanilla gnome-core gdm ghostty nm xdg-user-dirs |
| system management | gnome_debloat | remove 23 gnome apps and ptyxis |
| networking and security | | empty for now |
| development toolchain | dev_toolchain | python c c++ rust java nodejs dbs web tooling |
| performance and gaming | | empty for now |
| environment | | empty for now |

## controls

| key | action |
|---|---|
| tab / shift tab | next / previous task |
| up down | navigate or scroll |
| left right | navigate within item |
| enter | activate |
| esc | go back |
| ctrl c | quit safe modes cancel unsafe modes |
| / | fuzzy search |
| ? | help overlay |

no mouse keyboard only tui only

## license

gnu affero general public license v30 see LICENSE
