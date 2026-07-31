// snap_debloat rendering
pub fn render(
    state: &super::state::State,
    area: crate::core::rect::Rect,
    grid: &mut crate::core::grid::Grid,
) {
    let fg = crate::core::color::Color::Foreground;
    let bg = crate::core::color::Color::Background;
    let accent = crate::core::color::Color::Accent;
    let muted = crate::core::color::Color::Muted;

    grid.write_str(area.x, area.y, "Snap Debloat", accent, bg, true);
    grid.write_str(
        area.x,
        area.y + 2,
        "remove snapd and prevent it from being reinstalled",
        muted,
        bg,
        false,
    );

    let remove_box = if state.remove_snapd { "[x]" } else { "[ ]" };
    let pin_box = if state.pin_snapd { "[x]" } else { "[ ]" };

    let (remove_fg, _) = if state.selected_index == 0 {
        (accent, bg)
    } else {
        (fg, bg)
    };
    let (pin_fg, _) = if state.selected_index == 1 {
        (accent, bg)
    } else {
        (fg, bg)
    };

    grid.write_str(
        area.x,
        area.y + 4,
        &format!("{} remove snapd", remove_box),
        remove_fg,
        bg,
        state.selected_index == 0,
    );
    grid.write_str(
        area.x,
        area.y + 5,
        "  purge snapd and all snap packages",
        muted,
        bg,
        false,
    );

    grid.write_str(
        area.x,
        area.y + 7,
        &format!("{} pin snapd", pin_box),
        pin_fg,
        bg,
        state.selected_index == 1,
    );
    grid.write_str(
        area.x,
        area.y + 8,
        "  write apt pin to block snapd from reinstalling",
        muted,
        bg,
        false,
    );

    grid.write_str(
        area.x,
        area.y + 10,
        "press enter to run selected operations",
        muted,
        bg,
        false,
    );
}
