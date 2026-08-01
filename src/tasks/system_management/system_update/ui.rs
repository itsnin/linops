// system_update rendering
// shows two toggles update and upgrade
pub fn render(
    state: &super::state::State,
    area: crate::core::rect::Rect,
    grid: &mut crate::core::grid::Grid,
) {
    let fg = crate::core::color::Color::Foreground;
    let bg = crate::core::color::Color::Background;
    let accent = crate::core::color::Color::Accent;
    let muted = crate::core::color::Color::Muted;

    grid.write_str(area.x, area.y, "System Update", accent, bg, true);
    grid.write_str(
        area.x,
        area.y + 2,
        "update package lists and upgrade installed packages",
        muted,
        bg,
        false,
    );

    let update_box = if state.do_update { "[x]" } else { "[ ]" };
    let upgrade_box = if state.do_upgrade { "[x]" } else { "[ ]" };

    let (update_fg, _) = if state.selected_index == 0 {
        (accent, bg)
    } else {
        (fg, bg)
    };
    let (upgrade_fg, _) = if state.selected_index == 1 {
        (accent, bg)
    } else {
        (fg, bg)
    };

    grid.write_str(
        area.x,
        area.y + 4,
        &format!("{} apt update", update_box),
        update_fg,
        bg,
        state.selected_index == 0,
    );
    grid.write_str(
        area.x,
        area.y + 5,
        "  fetch the latest package lists from repositories",
        muted,
        bg,
        false,
    );

    grid.write_str(
        area.x,
        area.y + 7,
        &format!("{} apt upgrade", upgrade_box),
        upgrade_fg,
        bg,
        state.selected_index == 1,
    );
    grid.write_str(
        area.x,
        area.y + 8,
        "  upgrade all installed packages to latest versions",
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
