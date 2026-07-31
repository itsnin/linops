pub fn render(
    state: &super::state::State,
    area: crate::core::rect::Rect,
    grid: &mut crate::core::grid::Grid,
) {
    let fg = crate::core::color::Color::Foreground;
    let bg = crate::core::color::Color::Background;
    let accent = crate::core::color::Color::Accent;
    let muted = crate::core::color::Color::Muted;

    let names = super::state::State::names();
    grid.write_str(area.x, area.y, "Virtualization", accent, bg, true);
    grid.write_str(
        area.x,
        area.y + 2,
        "install virtualization tools",
        muted,
        bg,
        false,
    );

    for (i, (name, desc)) in names.iter().enumerate() {
        let row = area.y + 4 + i as u16 * 2;
        if row >= area.y + area.height - 1 {
            break;
        }
        let box_char = if state.is_checked(i) { "[x]" } else { "[ ]" };
        let row_fg = if state.selected_index == i {
            accent
        } else {
            fg
        };
        grid.write_str(
            area.x,
            row,
            &format!("{} {}", box_char, name),
            row_fg,
            bg,
            state.selected_index == i,
        );
        grid.write_str(area.x + 2, row + 1, desc, muted, bg, false);
    }
}
