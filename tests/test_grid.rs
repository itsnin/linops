use linops::core::color::Color;
use linops::core::grid::{Cell, Grid};

#[test]
fn test_grid_new_is_empty() {
    let grid = Grid::new(10, 5);
    assert_eq!(grid.width, 10);
    assert_eq!(grid.height, 5);
    assert_eq!(grid.get(0, 0).unwrap().ch, ' ');
}

#[test]
fn test_grid_set_and_get() {
    let mut grid = Grid::new(10, 5);
    grid.set(
        3,
        2,
        Cell {
            ch: 'x',
            fg: Color::Accent,
            bg: Color::Background,
            bold: true,
            underline: false,
        },
    );
    let r = grid.get(3, 2).unwrap();
    assert_eq!(r.ch, 'x');
    assert_eq!(r.fg, Color::Accent);
}

#[test]
fn test_grid_write_str() {
    let mut grid = Grid::new(20, 5);
    grid.write_str(2, 1, "hello", Color::Foreground, Color::Background, false);
    assert_eq!(grid.get(2, 1).unwrap().ch, 'h');
    assert_eq!(grid.get(6, 1).unwrap().ch, 'o');
}

#[test]
fn test_grid_write_str_stops_at_edge() {
    let mut grid = Grid::new(5, 3);
    grid.write_str(
        3,
        0,
        "hello world",
        Color::Foreground,
        Color::Background,
        false,
    );
    // only 2 chars fit starting at x=3 in a width=5 grid
    assert_eq!(grid.get(3, 0).unwrap().ch, 'h');
    assert_eq!(grid.get(4, 0).unwrap().ch, 'e');
}
