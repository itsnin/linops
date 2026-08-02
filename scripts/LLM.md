# LLM.md

instructions for any ai model (claude another model or a future
session of this same model) working on linops read this file in
full before touching anything treat it as binding not advisory

## what linops is

a modular tui for linux system management ubuntu first written in
rust with ratatui distro independent long term but ubuntu only for
v1 distributed as a single binary via curl | sh see scripts/start.sh
no rust no packages required on the target machine to run it

## the one rule that overrides everything else in this file

every task is exactly one .sh file nothing else

no state.rs no keys.rs no ui.rs no actions.rs no search.rs no
presets directory none of that per task rust scaffolding existed
earlier in this project and was deleted on purpose do not recreate
it even if you find it useful or elegant the user was explicit
about this multiple times in the same session including after
seeing a partially built version of the per task rust approach and
rejecting it

a task folder looks like this and only this

    src/tasks/<category>/<task_name>/<task_name>.sh

that is the whole task one bash file living in one folder named
after it if you are about to create a second file inside a task
folder stop you are doing it wrong

## why bash specifically not a portable posix sh

the user's own words every real linux install has bash available
even when it is not the login shell so there is no multi shell
compatibility burden here just always invoke bash directly do not
write #!/bin/sh do not write dash or ash compatible code do not
avoid bashisms write plain bash

## why .sh files instead of rust structs implementing a Task trait

this was tried first a Task trait with state update view methods
per task an Action enum InstallPackages RemovePackages HoldPackages
etc a DistroAdapter trait with one impl per distro all of it got
deleted the reasoning the user gave

- writing a new rust match arm and adapter method for every apt
  verb before a task can do anything is slow the goal is reaching a
  working task quickly not modeling every apt verb in rust's type
  system
- a verified bash one liner is faster to write test and port from
  an already working script than a new rust file per concern
- rust re deciding in a checkbox what a bash script already decides
  internally (see the linutil case study below) is duplicated logic
  not safety

## what stays in rust and why

the tui shell itself grid rendering color palette charset rules key
event reading terminal setup distro detection for display purposes
this is the part that has to stay swappable later per the user's
portability requirement getting stuck to ratatui specifically is
not acceptable if something better than ratatui exists later the
render layer should be replaceable without touching every task
that is exactly why tasks are plain .sh files and not ratatui
widgets a .sh file has zero opinion about how it is displayed

the tui's job reduced to its actual minimum
1 discover every <task_name>.sh file under src/tasks/<category>/
2 render them as a list grouped by category
3 arrow keys move a selection cursor enter runs the selected script
4 the script's stdout stderr and exit code are what the user sees
   there is no rust side mirror of what the script does internally

as of this writing the rust side for step 1 to 4 above still needs
a rewrite the old src/core/task.rs Task trait and the old
src/tasks/mod.rs builtin_tasks() registry both assumed rust structs
per task and no longer compile this is expected mid restructure not
a bug see status section at the bottom

## execution model root privilege and where sudo lives

the tui process itself never runs as root only the specific script
the user chooses to run gets escalated and only for that one
invocation this mirrors how the original debloat.sh re execs itself
as root rather than assuming the whole session is root

when a script needs root the runner invokes it as

    sudo bash path/to/task.sh

when it does not need root plain

    bash path/to/task.sh

scripts should not put sudo in front of every internal line the
whole script already runs as root once invoked via sudo bash internal
sudo prefixes are redundant not harmful but redundant confirm the
project's actual current mechanism for whether a task needs root
before assuming there is a metadata file or a naming convention for
this the rust side rewrite mentioned above has not settled this yet

## the linutil case study read this before writing any task script

the user pointed to ChrisTitusTech/linutil https://github.com/ChrisTitusTech/linutil
as the reference for how a script should be shaped specifically
core/tabs/system-setup/remove-snaps.sh

    #!/bin/sh -e
    . ../common-script.sh
    removeSnaps() {
        if command_exists snap; then
            case "$PACKAGER" in
                pacman)
                    "$ESCALATION_TOOL" "$PACKAGER" -Rns snapd --noconfirm
                    ;;
                apt-get|nala)
                    "$ESCALATION_TOOL" "$PACKAGER" remove --purge -y snapd
                    "$ESCALATION_TOOL" "$PACKAGER" autoremove -y
                    if [ "$ID" = ubuntu ]; then
                        "$ESCALATION_TOOL" apt-mark hold snapd
                    fi
                    ;;
                dnf|zypper)
                    "$ESCALATION_TOOL" "$PACKAGER" remove -y snapd
                    ;;
                *)
                    printf "%b\n" "${RED}Unsupported package manager: ""$PACKAGER""${RC}"
                    exit 1
                    ;;
            esac
            printf "%b\n" "${GREEN}Successfully removed snaps.${RC}"
        else
            printf "%b\n" "${GREEN}Snapd is not installed.${RC}"
        fi
    }
    checkEnv
    checkEscalationTool
    removeSnaps

what to actually take from this (linops uses bash not posix sh see
the rule above so do not copy the shebang or common-script.sh
sourcing verbatim the shape is what matters)

- idempotency guard first check if the thing is already done or
  already absent before acting command_exists snap before removing
  it print a message either way so the user always sees output
- one script one job the script decides everything about its own
  execution internally there is no external rust state mirroring
  what the script's if branches already decide
- clear success/failure output printed by the script itself not
  inferred by rust from exit codes alone

what not to take from this project is ubuntu only for v1 so do not
build the full multi distro case "$PACKAGER" in branching structure
yet a single distro's commands directly is fine leave a comment
noting other distros would branch here if this expands later do not
build the branching preemptively that is the same over engineering
the user has repeatedly pushed back on across this session

## verified scripts treat as ground truth do not re verify

these two files were given directly by the user as already tested
on their own real hardware confirmed working do not question
whether they work do not spend effort re verifying them just port
the exact commands faithfully

snapd.sh already ported into
src/tasks/system_management/snap_debloat/snapd.sh

    sudo apt-get remove --purge -y snapd
    sudo apt-get autoremove -y
    sudo apt-mark hold snapd

debloat.sh a larger multi section script that converts a fresh
ubuntu server install into a usable gnome desktop covers snap
removal apt pin equivalent now apt-mark hold not a pin file per an
explicit correction from the user apt cleanup kdump removal network
manager setup dev toolchain install upstream gnome install gnome
debloat and a cursor theme step

one exception inside debloat.sh the breeze cursor theme step the
user has explicitly said this specific part does not actually work
on their hardware even though the rest of the script is confirmed
working port it faithfully anyway since it is still the source
script's content but do not present it as verified working the way
every other section is and do not spend time trying to fix or debug
why it fails that is not requested

apt-mark hold replaces every pin file mechanism anywhere in this
project not just snapd the user was explicit the package names
themselves stay exactly as given in the verified scripts only the
mechanism (hold instead of a written apt preferences file) changes

## category structure current as of this writing

    1 package management   - web_browser system_util fonts
                              virtualization editor gnome_ext
    2 system management     - snap_debloat apt_pins system_update
                              apt_cleanup kdump_removal upstream_gnome
                              gnome_debloat
    3 networking security   - empty placeholder shows nothing yet
    4 development toolchain - dev_toolchain
    5 performance gaming    - empty placeholder shows nothing yet
    6 environment           - empty placeholder shows nothing yet

empty categories should exist and be visible in the ui but contain
no tasks yet do not populate them speculatively

## things explicitly rejected during this project do not reintroduce

- gui for any platform other than a future separate macos/windows
  tool linops itself is tui only on linux always
- getting the tui permanently locked to ratatui specifically the
  render layer must stay swappable
- a pin file apt preferences mechanism for blocking reinstalls use
  apt-mark hold
- per task rust state machines checkboxes toggled in rust that
  mirror what a bash script's own if branches already decide
- snap or flatpak as an install recommendation for anything the
  user has a standing preference against suggesting either
- building full multi distro branching before it is actually needed
  ubuntu only until the user says otherwise

## status as of this writing mid restructure

done
- src/tasks/**/*.sh scaffolded one file per task folder old rust
  files (state.rs keys.rs ui.rs actions.rs search.rs presets/)
  deleted from every task folder
- snap_debloat/snapd.sh has real verified content every other .sh
  file is a scaffold comment only no real commands yet
- src/core/distro/adapter.rs and src/core/distro/adapters/ deleted
  entirely the DistroAdapter trait no longer exists
- src/core/action.rs collapsed toward a RunScript style shape this
  may still be mid change confirm its actual current shape before
  relying on it
- background color bug fixed verified via running binary confirmed
  live on main
- navigation bug fixed up down now scrolls within whatever list is
  focused left right switches between sibling items matches the
  linutil focus model at the input dispatch level this fix lives in
  src/core/handlers/ which itself may be deleted or heavily changed
  once the .sh discovery mechanism replaces the old Task trait
  dispatch confirm current state before assuming this still applies

not done yet
- src/core/task.rs Task trait and src/tasks/mod.rs builtin_tasks()
  registry both still reference deleted rust structs and do not
  compile this needs a full rewrite around discovering .sh files by
  walking src/tasks/<category>/<task_name>/*.sh rather than
  registering rust struct instances
- the actual mechanism for how the runner decides whether a given
  .sh file needs sudo is not settled a metadata convention (file
  naming a sibling file a header comment the runner greps for)
  needs to be decided before this is real
- almost every task's .sh file is still a scaffold comment only real
  debloat.sh command content has not been ported into most of them
  yet only snapd.sh is real
- src/core/mode.rs src/core/dispatch.rs src/core/handlers/ all exist
  to route keys into per task rust state that no longer exists they
  very likely need to shrink to something much simpler a plain list
  cursor and an enter to run this has not been done yet confirm
  before assuming the current dispatch code is still the right shape

before adding anything re read this file's status section and the
actual current file tree do not assume a past summary especially
one from earlier in a long conversation is still accurate confirm
against what is really on disk first
