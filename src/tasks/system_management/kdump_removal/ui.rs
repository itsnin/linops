pub fn render(
    state: &super::state::State,
    area: crate::core::rect::Rect,
    grid: &mut crate::core::grid::Grid,
) {
    let accent = crate::core::color::Color::Accent;
    let muted = crate::core::color::Color::Muted;
    let bg = crate::core::color::Color::Background;

    grid.write_str(area.x, area.y, "Kdump Removal", accent, bg, true);
    grid.write_str(
        area.x,
        area.y + 2,
        "remove kdump-tools to free ~512mb of reserved kernel memory",
        muted,
        bg,
        false,
    );

    let box_char = if state.remove_kdump { "[x]" } else { "[ ]" };
    grid.write_str(
        area.x,
        area.y + 4,
        &format!("{} remove kdump-tools", box_char),
        accent,
        bg,
        true,
    );
    grid.write_str(
        area.x,
        area.y + 5,
        "  purges kdump-tools kexec-tools and removes grub config",
        muted,
        bg,
        false,
    );
    grid.write_str(
        area.x,
        area.y + 7,
        "press enter to toggle",
        muted,
        bg,
        false,
    );
}
