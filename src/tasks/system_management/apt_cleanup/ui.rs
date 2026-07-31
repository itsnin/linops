pub fn render(
    state: &super::state::State,
    area: crate::core::rect::Rect,
    grid: &mut crate::core::grid::Grid,
) {
    let fg = crate::core::color::Color::Foreground;
    let bg = crate::core::color::Color::Background;
    let accent = crate::core::color::Color::Accent;
    let muted = crate::core::color::Color::Muted;

    grid.write_str(area.x, area.y, "APT Cleanup", accent, bg, true);
    grid.write_str(
        area.x,
        area.y + 2,
        "mark critical packages manual drop metapackage autoremove",
        muted,
        bg,
        false,
    );

    let items = [
        (
            "mark manual",
            "mark critical gnome packages as manually installed",
            state.mark_manual,
        ),
        (
            "drop gnome-core",
            "remove the gnome-core metapackage wrapper",
            state.drop_gnome_core,
        ),
        (
            "autoremove purge",
            "remove orphaned packages and clean up",
            state.autoremove,
        ),
    ];

    for (i, (name, desc, checked)) in items.iter().enumerate() {
        let box_char = if *checked { "[x]" } else { "[ ]" };
        let row_fg = if state.selected_index == i {
            accent
        } else {
            fg
        };
        grid.write_str(
            area.x,
            area.y + 4 + i as u16 * 2,
            &format!("{} {}", box_char, name),
            row_fg,
            bg,
            state.selected_index == i,
        );
        grid.write_str(
            area.x + 2,
            area.y + 5 + i as u16 * 2,
            desc,
            muted,
            bg,
            false,
        );
    }

    grid.write_str(
        area.x,
        area.y + 12,
        "press enter to toggle",
        muted,
        bg,
        false,
    );
}
