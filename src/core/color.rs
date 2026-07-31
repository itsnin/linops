// 16 colors only because the linux kernel console vt only supports 16
// the kernel console uses vga compatible sgr codes 30-37 40-47 90-97 100-107
// ref https://www.kernel.org/doc/html/latest/fb/fbcon.html
// ref https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
//
// even on modern terminals we use 16 colors not truecolor
// this keeps the look identical everywhere
// the terminal palette decides the exact rgb but our semantic names stay the same
//
// ansi 16 color codes
// foreground 30-37 normal 90-97 bright
// background 40-47 normal 100-107 bright
// 0 black 1 red 2 green 3 yellow 4 blue 5 magenta 6 cyan 7 white
// 8 bright black gray 9 bright red 10 bright green 11 bright yellow
// 12 bright blue 13 bright magenta 14 bright cyan 15 bright white
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Background,
    Surface,
    Foreground,
    Muted,
    Accent,
    Border,
    Success,
    Warning,
    Error,
    Info,
    Purple,
    Orange,
    Green,
    Red,
    Yellow,
    Cyan,
}

impl Color {
    // returns the base ansi color index 0-7
    // 0 black 1 red 2 green 3 yellow 4 blue 5 magenta 6 cyan 7 white
    fn base(self) -> u8 {
        match self {
            Color::Background => 0,
            Color::Surface => 0,
            Color::Foreground => 7,
            Color::Muted => 7,
            Color::Accent => 4,
            Color::Border => 7,
            Color::Success => 2,
            Color::Warning => 3,
            Color::Error => 1,
            Color::Info => 6,
            Color::Purple => 5,
            Color::Orange => 1,
            Color::Green => 2,
            Color::Red => 1,
            Color::Yellow => 3,
            Color::Cyan => 6,
        }
    }

    // returns true if this color should use the bright variant
    fn is_bright(self) -> bool {
        match self {
            Color::Background => false,
            Color::Surface => true,
            Color::Foreground => true,
            Color::Muted => false,
            Color::Accent => false,
            Color::Border => false,
            Color::Success => false,
            Color::Warning => false,
            Color::Error => false,
            Color::Info => false,
            Color::Purple => false,
            Color::Orange => true,
            Color::Green => true,
            Color::Red => true,
            Color::Yellow => true,
            Color::Cyan => true,
        }
    }

    // returns the ansi sgr code for this color as foreground
    // normal 30-37 bright 90-97
    pub fn ansi_fg(self) -> u8 {
        if self.is_bright() {
            90 + self.base()
        } else {
            30 + self.base()
        }
    }

    // returns the ansi sgr code for this color as background
    // normal 40-47 bright 100-107
    pub fn ansi_bg(self) -> u8 {
        if self.is_bright() {
            100 + self.base()
        } else {
            40 + self.base()
        }
    }

    // returns the rgb triple
    // background is apple dark gray #1d1d1f not pure black
    // surface is slightly darker #161617
    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Background => (29, 29, 31),
            Color::Surface => (22, 22, 23),
            Color::Foreground => (192, 202, 245),
            Color::Muted => (108, 112, 134),
            Color::Accent => (122, 162, 247),
            Color::Border => (58, 60, 68),
            Color::Success => (158, 206, 106),
            Color::Warning => (224, 175, 104),
            Color::Error => (247, 118, 142),
            Color::Info => (125, 207, 255),
            Color::Purple => (187, 154, 247),
            Color::Orange => (255, 158, 100),
            Color::Green => (115, 218, 202),
            Color::Red => (219, 75, 75),
            Color::Yellow => (224, 175, 104),
            Color::Cyan => (125, 207, 255),
        }
    }
}
