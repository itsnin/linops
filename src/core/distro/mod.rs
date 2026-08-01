// distro abstraction layer
// modules call abstract methods on the adapter
// the adapter translates to the right command for the detected distro
// this is why tasks never call apt or dnf directly
pub mod adapter;
pub mod adapters;
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
