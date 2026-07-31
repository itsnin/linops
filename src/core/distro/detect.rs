// parse /etc/os-release to detect the running distro
// os-release is the freedesktop standard present on every modern linux
// ref https://www.freedesktop.org/software/systemd/man/os-release.html
//
// we match the id field and fall back to id_like
// id_like is how derivatives declare their base distro
// for example linux mint has id=mint and id_like=ubuntu
pub fn detect() -> crate::core::distro::DistroId {
    let content = match std::fs::read_to_string("/etc/os-release") {
        Ok(c) => c,
        Err(_) => return crate::core::distro::DistroId::Unknown,
    };

    let mut id = String::new();
    let mut id_like = String::new();

    for line in content.lines() {
        if let Some(val) = line.strip_prefix("ID=") {
            id = val.trim_matches('"').to_string();
        } else if let Some(val) = line.strip_prefix("ID_LIKE=") {
            id_like = val.trim_matches('"').to_string();
        }
    }

    // check id first
    match id.as_str() {
        "ubuntu" => return crate::core::distro::DistroId::Ubuntu,
        "debian" => return crate::core::distro::DistroId::Debian,
        "fedora" => return crate::core::distro::DistroId::Fedora,
        "arch" => return crate::core::distro::DistroId::Arch,
        "opensuse-leap" | "opensuse-tumbleweed" => return crate::core::distro::DistroId::OpenSuse,
        "alpine" => return crate::core::distro::DistroId::Alpine,
        _ => {}
    }

    // check id_like for derivatives
    for like in id_like.split_whitespace() {
        match like {
            "ubuntu" => return crate::core::distro::DistroId::Ubuntu,
            "debian" => return crate::core::distro::DistroId::Debian,
            "fedora" | "rhel" => return crate::core::distro::DistroId::Fedora,
            "arch" => return crate::core::distro::DistroId::Arch,
            _ => {}
        }
    }

    crate::core::distro::DistroId::Unknown
}
