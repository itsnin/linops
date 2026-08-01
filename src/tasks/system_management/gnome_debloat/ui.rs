pub fn render(
    state: &super::state::State,
    area: crate::core::rect::Rect,
    grid: &mut crate::core::grid::Grid,
) {
    let fg = crate::core::color::Color::Foreground;
    let bg = crate::core::color::Color::Background;
    let accent = crate::core::color::Color::Accent;
    let muted = crate::core::color::Color::Muted;

    grid.write_str(area.x, area.y, "GNOME Debloat", accent, bg, true);
    grid.write_str(
        area.x,
        area.y + 2,
        "remove gnome utility apps and ptyxis terminal",
        muted,
        bg,
        false,
    );

    let apps_box = if state.remove_gnome_apps {
        "[x]"
    } else {
        "[ ]"
    };
    let ptyxis_box = if state.remove_ptyxis { "[x]" } else { "[ ]" };

    let (apps_fg, _) = if state.selected_index == 0 {
        (accent, bg)
    } else {
        (fg, bg)
    };
    let (ptyxis_fg, _) = if state.selected_index == 1 {
        (accent, bg)
    } else {
        (fg, bg)
    };

    grid.write_str(
        area.x,
        area.y + 4,
        &format!("{} remove gnome apps (23 packages)", apps_box),
        apps_fg,
        bg,
        state.selected_index == 0,
    );
    grid.write_str(
        area.x + 2,
        area.y + 5,
        "calculator calendar characters clocks contacts disk-utility",
        muted,
        bg,
        false,
    );
    grid.write_str(
        area.x + 2,
        area.y + 6,
        "font-viewer logs maps weather sushi system-monitor",
        muted,
        bg,
        false,
    );
    grid.write_str(
        area.x + 2,
        area.y + 7,
        "text-editor baobab loupe papers showtime simple-scan",
        muted,
        bg,
        false,
    );
    grid.write_str(
        area.x + 2,
        area.y + 8,
        "connections user-docs yelp orca gnome-software",
        muted,
        bg,
        false,
    );

    grid.write_str(
        area.x,
        area.y + 10,
        &format!("{} remove ptyxis", ptyxis_box),
        ptyxis_fg,
        bg,
        state.selected_index == 1,
    );
    grid.write_str(
        area.x + 2,
        area.y + 11,
        "ubuntus custom terminal replaced by ghostty",
        muted,
        bg,
        false,
    );

    grid.write_str(
        area.x,
        area.y + 13,
        "press enter to toggle",
        muted,
        bg,
        false,
    );
}
