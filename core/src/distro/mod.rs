// distro detection only
// execution goes through bash scripts embedded per task not a rust
// adapter see core/action.rs for why
pub mod detect;

// distro id is detected from /etc/os-release
// only ubuntu for now others added later
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DistroId {
    Ubuntu,
    Debian,
    Fedora,
    Arch,
    OpenSuse,
    Alpine,
    Unknown,
}
