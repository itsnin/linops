// one cell is one character position on screen
// both tui and gui render from the same grid so they look identical
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub ch: char,
    pub fg: crate::color::Color,
    pub bg: crate::color::Color,
    pub bold: bool,
    pub underline: bool,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            ch: ' ',
            fg: crate::color::Color::Foreground,
            bg: crate::color::Color::Background,
            bold: false,
            underline: false,
        }
    }
}

// a fixed size grid of cells
// the view function fills this and the renderers read it
pub struct Grid {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let cells = vec![Cell::empty(); (width as usize) * (height as usize)];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        self.cells
            .get((y as usize) * (self.width as usize) + (x as usize))
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(target) = self
            .cells
            .get_mut((y as usize) * (self.width as usize) + (x as usize))
        {
            *target = cell;
        }
    }

    // write one row of an item list applying the shared selection look
    // selected rows get a full width background fill black text bold
    // and a > prefix unselected rows are plain text with a blank prefix
    // this matches how a real list widget (ratatui List + ListState with
    // highlight_style highlight_symbol) draws its selection so the same
    // visual result holds even though this is a plain grid write and not
    // a stateful list widget which keeps task code renderer agnostic
    // ref linutil tui/src/state.rs list_highlight_style list_highlight_symbol
    pub fn write_selectable_row(&mut self, x: u16, y: u16, width: u16, label: &str, selected: bool) {
        let prefix = if selected { "> " } else { "  " };
        let (fg, bg, bold) = if selected {
            (
                crate::color::Color::Background,
                crate::color::Color::Accent,
                true,
            )
        } else {
            (
                crate::color::Color::Foreground,
                crate::color::Color::Background,
                false,
            )
        };

        // fill the full row width first so the highlight bar has no gaps
        // past the label text then overwrite with the prefix and label
        self.fill(crate::rect::Rect::new(x, y, width, 1), bg);
        self.write_str(x, y, &format!("{prefix}{label}"), fg, bg, bold);
    }

    // write a string left to right starting at x y
    // stops at the grid edge so no overflow
    // characters outside the cp437 safe set are sanitized
    pub fn write_str(
        &mut self,
        x: u16,
        y: u16,
        s: &str,
        fg: crate::color::Color,
        bg: crate::color::Color,
        bold: bool,
    ) {
        for (col, ch) in (x..).zip(s.chars()) {
            if col >= self.width {
                break;
            }
            let safe_ch = crate::charset::sanitize(ch);
            self.set(
                col,
                y,
                Cell {
                    ch: safe_ch,
                    fg,
                    bg,
                    bold,
                    underline: false,
                },
            );
        }
    }

    // draw a single line box border around the given rect
    // uses cp437 box drawing chars so it works in the kernel console
    pub fn box_border(&mut self, rect: crate::rect::Rect) {
        let fg = crate::color::Color::Border;
        let bg = crate::color::Color::Background;
        let x2 = rect.x + rect.width.saturating_sub(1);
        let y2 = rect.y + rect.height.saturating_sub(1);

        // corners
        self.set(
            rect.x,
            rect.y,
            Cell {
                ch: '\u{250C}',
                fg,
                bg,
                bold: false,
                underline: false,
            },
        );
        self.set(
            x2,
            rect.y,
            Cell {
                ch: '\u{2510}',
                fg,
                bg,
                bold: false,
                underline: false,
            },
        );
        self.set(
            rect.x,
            y2,
            Cell {
                ch: '\u{2514}',
                fg,
                bg,
                bold: false,
                underline: false,
            },
        );
        self.set(
            x2,
            y2,
            Cell {
                ch: '\u{2518}',
                fg,
                bg,
                bold: false,
                underline: false,
            },
        );

        // top and bottom edges
        for x in (rect.x + 1)..x2 {
            self.set(
                x,
                rect.y,
                Cell {
                    ch: '\u{2500}',
                    fg,
                    bg,
                    bold: false,
                    underline: false,
                },
            );
            self.set(
                x,
                y2,
                Cell {
                    ch: '\u{2500}',
                    fg,
                    bg,
                    bold: false,
                    underline: false,
                },
            );
        }

        // left and right edges
        for y in (rect.y + 1)..y2 {
            self.set(
                rect.x,
                y,
                Cell {
                    ch: '\u{2502}',
                    fg,
                    bg,
                    bold: false,
                    underline: false,
                },
            );
            self.set(
                x2,
                y,
                Cell {
                    ch: '\u{2502}',
                    fg,
                    bg,
                    bold: false,
                    underline: false,
                },
            );
        }
    }

    // fill a rect with a solid color
    pub fn fill(&mut self, rect: crate::rect::Rect, bg: crate::color::Color) {
        for y in rect.y..(rect.y + rect.height).min(self.height) {
            for x in rect.x..(rect.x + rect.width).min(self.width) {
                self.set(
                    x,
                    y,
                    Cell {
                        ch: ' ',
                        fg: bg,
                        bg,
                        bold: false,
                        underline: false,
                    },
                );
            }
        }
    }
}
