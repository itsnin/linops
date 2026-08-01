pub fn render(
    state: &super::state::State,
    area: crate::core::rect::Rect,
    grid: &mut crate::core::grid::Grid,
) {
    let accent = crate::core::color::Color::Accent;
    let muted = crate::core::color::Color::Muted;
    let bg = crate::core::color::Color::Background;

    grid.write_str(area.x, area.y, "APT Pins", accent, bg, true);
    grid.write_str(
        area.x,
        area.y + 2,
        "write apt pin file to block removed packages from reinstalling",
        muted,
        bg,
        false,
    );

    let box_char = if state.write_pins { "[x]" } else { "[ ]" };
    grid.write_str(
        area.x,
        area.y + 4,
        &format!("{} write pin file", box_char),
        accent,
        bg,
        true,
    );
    grid.write_str(
        area.x,
        area.y + 5,
        "  writes to /etc/apt/preferences.d/block-gnome-bloat",
        muted,
        bg,
        false,
    );
    grid.write_str(
        area.x,
        area.y + 6,
        "  pin priority -1 blocks all listed packages permanently",
        muted,
        bg,
        false,
    );
    grid.write_str(
        area.x,
        area.y + 8,
        "press enter to toggle",
        muted,
        bg,
        false,
    );
}
