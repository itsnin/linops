// actions are what tasks return when the user interacts
// the core executes actions via the distro adapter
// tasks never call apt or dnf directly
//
// actions that need root are run via sudo on demand
// the tui itself never runs as root
#[derive(Clone, Debug)]
pub enum Action {
    // install packages using the distro package manager
    InstallPackages(Vec<String>),
    // remove packages
    RemovePackages(Vec<String>),
    // purge packages including config
    PurgePackages(Vec<String>),
    // mark packages as manually installed so autoremove does not touch them
    MarkManual(Vec<String>),
    // write an apt pin file to block packages from reinstalling
    WritePinFile {
        path: String,
        packages: Vec<String>,
    },
    // enable a systemd service
    EnableService(String),
    // disable a systemd service
    DisableService(String),
    // set a gsettings key as the logged in user
    SetGsettings {
        schema: String,
        key: String,
        value: String,
    },
    // run a shell command as root via the helper
    RunCommand {
        cmd: String,
        args: Vec<String>,
    },
    // no action used when a key press does not produce an action
    None,
}
