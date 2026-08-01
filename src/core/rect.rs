// a rectangle in the grid used for layout
// x y is the top left corner width and height extend right and down
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    // split horizontally into two rects
    // first gets left_width columns second gets the rest
    pub fn split_h(self, left_width: u16) -> (Rect, Rect) {
        let left = Rect::new(self.x, self.y, left_width, self.height);
        let right = Rect::new(
            self.x + left_width,
            self.y,
            self.width.saturating_sub(left_width),
            self.height,
        );
        (left, right)
    }

    // split vertically into two rects
    // first gets top_height rows second gets the rest
    pub fn split_v(self, top_height: u16) -> (Rect, Rect) {
        let top = Rect::new(self.x, self.y, self.width, top_height);
        let bottom = Rect::new(
            self.x,
            self.y + top_height,
            self.width,
            self.height.saturating_sub(top_height),
        );
        (top, bottom)
    }
}
