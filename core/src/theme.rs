// theme maps semantic color names to actual colors
// the same theme drives both tui and gui so the look is consistent
// theme is loaded from config toml at startup
// if no config exists a default theme is used
// this is part of the just works philosophy no config required
pub struct Theme {
    pub bg: crate::color::Color,
    pub surface: crate::color::Color,
    pub fg: crate::color::Color,
    pub muted: crate::color::Color,
    pub accent: crate::color::Color,
    pub border: crate::color::Color,
    pub success: crate::color::Color,
    pub warning: crate::color::Color,
    pub error: crate::color::Color,
    pub info: crate::color::Color,
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            bg: crate::color::Color::Background,
            surface: crate::color::Color::Surface,
            fg: crate::color::Color::Foreground,
            muted: crate::color::Color::Muted,
            accent: crate::color::Color::Accent,
            border: crate::color::Color::Border,
            success: crate::color::Color::Success,
            warning: crate::color::Color::Warning,
            error: crate::color::Color::Error,
            info: crate::color::Color::Info,
        }
    }
}
