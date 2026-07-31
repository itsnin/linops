// theme maps semantic color names to actual colors
// the same theme drives both tui and gui so the look is consistent
// theme is loaded from config toml at startup
// if no config exists a default theme is used
// this is part of the just works philosophy no config required
pub struct Theme {
    pub bg: crate::core::color::Color,
    pub surface: crate::core::color::Color,
    pub fg: crate::core::color::Color,
    pub muted: crate::core::color::Color,
    pub accent: crate::core::color::Color,
    pub border: crate::core::color::Color,
    pub success: crate::core::color::Color,
    pub warning: crate::core::color::Color,
    pub error: crate::core::color::Color,
    pub info: crate::core::color::Color,
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            bg: crate::core::color::Color::Background,
            surface: crate::core::color::Color::Surface,
            fg: crate::core::color::Color::Foreground,
            muted: crate::core::color::Color::Muted,
            accent: crate::core::color::Color::Accent,
            border: crate::core::color::Color::Border,
            success: crate::core::color::Color::Success,
            warning: crate::core::color::Color::Warning,
            error: crate::core::color::Color::Error,
            info: crate::core::color::Color::Info,
        }
    }
}
