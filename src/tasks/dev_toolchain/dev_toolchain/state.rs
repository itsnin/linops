// dev_toolchain state
// rust is installed via rustup not apt
#[derive(Default)]
pub struct State {
    pub checked: Vec<bool>,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        let count = Self::names().len();
        Self {
            checked: vec![true; count],
            selected_index: 0,
        }
    }

    // each entry is (display_name, description, install_method)
    // install_method: "apt" means apt-get install, "rustup" means curl rustup
    pub fn names() -> &'static [(&'static str, &'static str, &'static str)] {
        &[
            ("python", "python3 pip venv dev full ssl ffi zlib bz2 lzma readline sqlite ncurses", "apt"),
            ("c c++", "build-essential gcc g++ clang clangd clang-format clang-tidy make cmake ninja gdb pkg-config valgrind llvm", "apt"),
            ("rust", "rustc cargo rustfmt rust-clippy installed via rustup not apt", "rustup"),
            ("java", "default-jdk maven", "apt"),
            ("nodejs", "nodejs javascript runtime", "apt"),
            ("databases", "sqlite3 postgresql-client mariadb-client pgcli mycli", "apt"),
            ("web tooling", "tidy html-xml-utils sassc ca-certificates gnupg", "apt"),
        ]
    }

    pub fn item_count() -> usize {
        Self::names().len()
    }

    pub fn is_checked(&self, i: usize) -> bool {
        self.checked.get(i).copied().unwrap_or(false)
    }

    pub fn toggle(&mut self, i: usize) {
        if let Some(c) = self.checked.get_mut(i) {
            *c = !*c;
        }
    }
}
