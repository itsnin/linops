// 16 colors only because the linux kernel console vt only supports 16
// the kernel console uses vga compatible sgr codes 30-37 40-47 90-97 100-107
// ref https://www.kernel.org/doc/html/latest/fb/fbcon.html
// ref https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
//
// even on modern terminals we use 16 colors not truecolor
// this keeps the look identical everywhere
// the terminal palette decides the exact rgb but our semantic names stay the same
//
// ansi 16 color palette indices 0-15
// 0 black 1 red 2 green 3 yellow 4 blue 5 magenta 6 cyan 7 white
// 8 bright black gray 9 bright red 10 bright green 11 bright yellow
// 12 bright blue 13 bright magenta 14 bright cyan 15 bright white
//
// ratatui Color::Indexed expects 0-255 palette index
// so we return 0-15 for the 16 color palette
// the renderer converts these to the correct sgr escape codes
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
    // returns the 0-15 palette index for this color
    // this is what ratatui Color::Indexed expects
    // 0 black 1 red 2 green 3 yellow 4 blue 5 magenta 6 cyan 7 white
    // 8 bright black 9 bright red 10 bright green 11 bright yellow
    // 12 bright blue 13 bright magenta 14 bright cyan 15 bright white
    pub fn ansi_fg(self) -> u8 {
        match self {
            Color::Background => 0,
            Color::Surface => 8,
            Color::Foreground => 7,
            Color::Muted => 8,
            Color::Accent => 4,
            Color::Border => 8,
            Color::Success => 2,
            Color::Warning => 3,
            Color::Error => 1,
            Color::Info => 6,
            Color::Purple => 5,
            Color::Orange => 1,
            Color::Green => 10,
            Color::Red => 9,
            Color::Yellow => 11,
            Color::Cyan => 14,
        }
    }

    // background uses the same palette index as foreground
    // ratatui handles the fg/bg distinction internally
    pub fn ansi_bg(self) -> u8 {
        self.ansi_fg()
    }

    // returns the rgb triple
    // background is apple dark gray #1d1d1f not pure black
    // surface is slightly darker #161617
    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Background => (29, 29, 31),
            Color::Surface => (22, 22, 23),
            Color::Foreground => (208, 208, 208),
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
