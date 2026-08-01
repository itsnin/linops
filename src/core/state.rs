// app state is the single source of truth
// it holds the mode the active category the active task within that category
// and the search state
//
// sidebar shows only category names
// main area shows tasks of the selected category
// selecting a task within the main area activates it
pub struct AppState {
    pub mode: crate::core::mode::Mode,
    pub active_category: usize,
    pub active_task: usize,
    pub tasks: Vec<Box<dyn crate::core::task::Task>>,
    pub distro: Option<crate::core::distro::DistroId>,
    pub running: bool,
    pub log_lines: Vec<String>,
    pub current_task: Option<String>,
    pub task_progress: u8,
    pub search_query: String,
    pub search_all_items: Vec<(usize, crate::core::search::SearchItem)>,
    pub search_results: Vec<(usize, u32)>,
    pub search_selected: usize,
    pub width: u16,
    pub height: u16,
    pub pending_actions: Vec<crate::core::action::Action>,
}

// all categories in display order
pub const ALL_CATEGORIES: [crate::core::task::Category; 6] = [
    crate::core::task::Category::PackageManagement,
    crate::core::task::Category::SystemManagement,
    crate::core::task::Category::NetworkingSecurity,
    crate::core::task::Category::DevelopmentToolchain,
    crate::core::task::Category::PerformanceGaming,
    crate::core::task::Category::Environment,
];

impl AppState {
    pub fn new(tasks: Vec<Box<dyn crate::core::task::Task>>) -> Self {
        Self {
            mode: crate::core::mode::Mode::Normal,
            active_category: 0,
            active_task: 0,
            tasks,
            distro: None,
            running: true,
            log_lines: Vec::new(),
            current_task: None,
            task_progress: 0,
            search_query: String::new(),
            search_all_items: Vec::new(),
            search_results: Vec::new(),
            search_selected: 0,
            width: 80,
            height: 24,
            pending_actions: Vec::new(),
        }
    }

    // returns the category that is currently active in the sidebar
    pub fn current_category(&self) -> crate::core::task::Category {
        ALL_CATEGORIES[self.active_category.min(ALL_CATEGORIES.len() - 1)]
    }

    // returns indices of tasks that belong to the current category
    pub fn tasks_in_current_category(&self) -> Vec<usize> {
        let cat = self.current_category();
        self.tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.category() == cat)
            .map(|(i, _)| i)
            .collect()
    }

    // returns the active task if it belongs to the current category
    pub fn active_task(&self) -> Option<&dyn crate::core::task::Task> {
        let indices = self.tasks_in_current_category();
        indices
            .get(self.active_task)
            .and_then(|&i| self.tasks.get(i).map(|b| b.as_ref()))
    }

    pub fn active_task_mut(&mut self) -> Option<&mut dyn crate::core::task::Task> {
        let indices = self.tasks_in_current_category();
        if let Some(&idx) = indices.get(self.active_task) {
            let boxed: &mut Box<dyn crate::core::task::Task> = &mut self.tasks[idx];
            return Some(boxed.as_mut());
        }
        None
    }

    pub fn next_category(&mut self) {
        self.active_category = (self.active_category + 1) % ALL_CATEGORIES.len();
        self.active_task = 0;
    }

    pub fn prev_category(&mut self) {
        if self.active_category == 0 {
            self.active_category = ALL_CATEGORIES.len() - 1;
        } else {
            self.active_category -= 1;
        }
        self.active_task = 0;
    }

    pub fn next_task(&mut self) {
        let count = self.tasks_in_current_category().len();
        if count > 0 {
            self.active_task = (self.active_task + 1) % count;
        }
    }

    pub fn prev_task(&mut self) {
        let count = self.tasks_in_current_category().len();
        if count > 0 {
            if self.active_task == 0 {
                self.active_task = count - 1;
            } else {
                self.active_task -= 1;
            }
        }
    }

    pub fn rebuild_search_index(&mut self) {
        self.search_all_items.clear();
        for (i, task) in self.tasks.iter().enumerate() {
            for item in task.searchable_items() {
                self.search_all_items.push((i, item));
            }
        }
    }

    pub fn enter_search(&mut self) {
        self.mode = crate::core::mode::Mode::Search;
        self.search_query.clear();
        self.search_selected = 0;
        self.rebuild_search_index();
        self.update_search_results();
    }

    pub fn exit_search(&mut self) {
        self.mode = crate::core::mode::Mode::Normal;
        self.search_query.clear();
        self.search_results.clear();
    }

    pub fn update_search_results(&mut self) {
        let query = self.search_query.clone();
        let items: Vec<&crate::core::search::SearchItem> =
            self.search_all_items.iter().map(|(_, item)| item).collect();

        let matched = crate::core::search::search_results(&query, &items);

        self.search_results = matched
            .into_iter()
            .map(|(item, score)| {
                let task_index = self
                    .search_all_items
                    .iter()
                    .find(|(_, i)| std::ptr::eq(i, item))
                    .map(|(idx, _)| *idx)
                    .unwrap_or(0);
                (task_index, score)
            })
            .collect();

        if !self.search_results.is_empty() {
            self.search_selected = 0;
        }
    }

    pub fn jump_to_search_result(&mut self) {
        if let Some(&(task_index, _)) = self.search_results.get(self.search_selected) {
            // find which category this task belongs to
            if let Some(task) = self.tasks.get(task_index) {
                let cat = task.category();
                for (i, c) in ALL_CATEGORIES.iter().enumerate() {
                    if *c == cat {
                        self.active_category = i;
                        break;
                    }
                }
                // find the task index within that category
                let indices = self.tasks_in_current_category();
                if let Some(pos) = indices.iter().position(|&i| i == task_index) {
                    self.active_task = pos;
                }
            }
            self.exit_search();
        }
    }
}
