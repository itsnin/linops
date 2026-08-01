// converts a core grid into a ratatui buffer for rendering
// maps each cell character to ratatui and each color to ansi sgr code
// this is the only place ratatui buffer appears
// the rest of the code only sees core grid
use ratatui::buffer::Buffer;
use ratatui::layout::Rect as RatatuiRect;
use ratatui::style::{Color as RatatuiColor, Modifier, Style};

pub fn render_grid(grid: &crate::core::grid::Grid, buffer: &mut Buffer, area: RatatuiRect) {
    for y in 0..grid.height.min(area.height) {
        for x in 0..grid.width.min(area.width) {
            let cell = match grid.get(x, y) {
                Some(c) => c,
                None => continue,
            };

            let rat_x = area.x + x;
            let rat_y = area.y + y;

            let rat_cell = &mut buffer[(rat_x, rat_y)];

            rat_cell.set_symbol(&cell.ch.to_string());

            let fg = RatatuiColor::Indexed(cell.fg.ansi_fg());
            let bg = RatatuiColor::Indexed(cell.bg.ansi_bg());

            let mut style = Style::default().fg(fg).bg(bg);

            if cell.bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if cell.underline {
                style = style.add_modifier(Modifier::UNDERLINED);
            }

            rat_cell.set_style(style);
        }
    }
}
