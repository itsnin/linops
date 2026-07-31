#[derive(Default)]
pub struct State {
    pub checked: Vec<bool>,
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            checked: vec![true; 1],
            selected_index: 0,
        }
    }

    pub fn names() -> &'static [(&'static str, &'static str)] {
        &[(
            "micro",
            "modern terminal text editor with intuitive nano-style ctrl shortcuts",
        )]
    }

    pub fn item_count() -> usize {
        Self::names().len()
    }

    pub fn is_checked(&self, i: usize) -> bool {
        self.checked.get(i).copied().unwrap_or(false)
    }

    pub fn toggle(&mut self, i: usize) {
        if let Some(c) = self.checked.get_mut(i) {
            *c = !*c;
        }
    }
}
