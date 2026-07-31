// view takes the current state and produces a grid of cells
// the grid is what the tui renders
//
// layout
// topbar at the top 3 rows
// sidebar on the left shows only category names
// main content on the right shows tasks of the selected category
// statusbar at the bottom 1 row
//
// if terminal is smaller than 80x24 show a too small message
pub fn view(
    state: &crate::core::state::AppState,
    theme: &crate::core::theme::Theme,
) -> crate::core::grid::Grid {
    let mut grid = crate::core::grid::Grid::new(state.width, state.height);

    if state.width < 80 || state.height < 24 {
        grid.write_str(
            1,
            1,
            "terminal too small need at least 80x24",
            crate::core::color::Color::Error,
            theme.bg,
            true,
        );
        return grid;
    }

    // fill background
    grid.fill(
        crate::core::rect::Rect::new(0, 0, state.width, state.height),
        theme.bg,
    );

    // layout: topbar 3 rows then content then statusbar 1 row
    let (topbar, rest) = crate::core::rect::Rect::new(0, 0, state.width, state.height).split_v(3);
    let (content, statusbar) = rest.split_v(state.height.saturating_sub(3 + 1));

    // split content into sidebar and main
    let (sidebar, main) = content.split_h(26);

    render_topbar(&mut grid, &topbar, state, theme);
    render_sidebar(&mut grid, &sidebar, state, theme);
    render_main(&mut grid, &main, state, theme);
    render_statusbar(&mut grid, &statusbar, state, theme);

    grid
}

fn render_topbar(
    grid: &mut crate::core::grid::Grid,
    area: &crate::core::rect::Rect,
    state: &crate::core::state::AppState,
    theme: &crate::core::theme::Theme,
) {
    grid.fill(*area, theme.surface);
    grid.box_border(*area);

    let distro_name = state
        .distro
        .map(|d| format!("{:?}", d))
        .unwrap_or_else(|| "detecting...".to_string());

    let title = format!(" linops v2026.08.01 | {} ", distro_name);
    grid.write_str(
        area.x + 1,
        area.y + 1,
        &title,
        theme.accent,
        theme.surface,
        true,
    );

    let mode_str = match state.mode {
        crate::core::mode::Mode::Normal => " NORMAL ",
        crate::core::mode::Mode::Search => " SEARCH ",
        crate::core::mode::Mode::Confirm => " CONFIRM ",
        crate::core::mode::Mode::Help => " HELP ",
        crate::core::mode::Mode::Task => " RUNNING ",
    };
    grid.write_str(
        area.x + area.width.saturating_sub(mode_str.len() as u16 + 2),
        area.y + 1,
        mode_str,
        theme.warning,
        theme.surface,
        true,
    );
}

fn render_sidebar(
    grid: &mut crate::core::grid::Grid,
    area: &crate::core::rect::Rect,
    state: &crate::core::state::AppState,
    theme: &crate::core::theme::Theme,
) {
    grid.fill(*area, theme.surface);
    grid.box_border(*area);

    grid.write_str(
        area.x + 1,
        area.y,
        " Categories ",
        theme.muted,
        theme.surface,
        false,
    );

    for (row_offset, (i, cat)) in (0..).zip(crate::core::state::ALL_CATEGORIES.iter().enumerate()) {
        let row = area.y + 1 + row_offset as u16;
        if row >= area.y + area.height - 1 {
            break;
        }

        let is_active = i == state.active_category;
        let prefix = if is_active { ">" } else { " " };
        let name = cat.display_name();

        let fg = if is_active { theme.accent } else { theme.fg };

        let label = format!("{} {}", prefix, name);
        grid.write_str(area.x + 1, row, &label, fg, theme.surface, is_active);
    }
}

fn render_main(
    grid: &mut crate::core::grid::Grid,
    area: &crate::core::rect::Rect,
    state: &crate::core::state::AppState,
    theme: &crate::core::theme::Theme,
) {
    grid.fill(*area, theme.bg);
    grid.box_border(*area);

    match state.mode {
        crate::core::mode::Mode::Search => {
            render_search(grid, area, state, theme);
        }
        crate::core::mode::Mode::Help => {
            render_help(grid, area, state, theme);
        }
        _ => {
            let cat = state.current_category();
            grid.write_str(
                area.x + 1,
                area.y,
                &format!(" {} ", cat.display_name()),
                theme.muted,
                theme.bg,
                false,
            );

            if !cat.has_tasks() {
                grid.write_str(
                    area.x + 1,
                    area.y + 1,
                    "no tasks in this category yet",
                    theme.muted,
                    theme.bg,
                    false,
                );
                return;
            }

            // show the active task's own ui
            if let Some(task) = state.active_task() {
                let inner = crate::core::rect::Rect::new(
                    area.x + 1,
                    area.y + 1,
                    area.width.saturating_sub(2),
                    area.height.saturating_sub(2),
                );
                task.render(inner, grid);
            } else {
                grid.write_str(
                    area.x + 1,
                    area.y + 1,
                    "no task selected",
                    theme.muted,
                    theme.bg,
                    false,
                );
            }
        }
    }
}

fn render_search(
    grid: &mut crate::core::grid::Grid,
    area: &crate::core::rect::Rect,
    state: &crate::core::state::AppState,
    theme: &crate::core::theme::Theme,
) {
    let prompt = format!("/ {}", state.search_query);
    grid.write_str(
        area.x + 1,
        area.y + 1,
        &prompt,
        theme.accent,
        theme.bg,
        true,
    );

    if state.search_results.is_empty() && !state.search_query.is_empty() {
        grid.write_str(
            area.x + 1,
            area.y + 3,
            "no results",
            theme.muted,
            theme.bg,
            false,
        );
        return;
    }

    let mut row = area.y + 3;
    for (i, &(task_idx, _)) in state.search_results.iter().enumerate() {
        if row >= area.y + area.height - 1 {
            break;
        }
        if let Some(item) = state
            .search_all_items
            .iter()
            .find(|(idx, _)| *idx == task_idx)
        {
            let (fg, bg) = if i == state.search_selected {
                (theme.accent, theme.bg)
            } else {
                (theme.fg, theme.bg)
            };
            let prefix = if i == state.search_selected { ">" } else { " " };
            let label = format!("{} {}", prefix, item.1.label);
            grid.write_str(area.x + 1, row, &label, fg, bg, i == state.search_selected);
            row += 1;
        }
    }
}

fn render_help(
    grid: &mut crate::core::grid::Grid,
    area: &crate::core::rect::Rect,
    _state: &crate::core::state::AppState,
    theme: &crate::core::theme::Theme,
) {
    let lines = [
        "linops help",
        "",
        "tab       next category",
        "shift tab prev category",
        "up down   navigate tasks",
        "enter     activate task",
        "esc       go back",
        "ctrl c    quit or cancel",
        "/         search",
        "?         this help",
        "",
        "press ? or esc to close",
    ];

    for (row, line) in (area.y + 1..).zip(lines.iter()) {
        if row >= area.y + area.height - 1 {
            break;
        }
        grid.write_str(area.x + 1, row, line, theme.fg, theme.bg, false);
    }
}

fn render_statusbar(
    grid: &mut crate::core::grid::Grid,
    area: &crate::core::rect::Rect,
    state: &crate::core::state::AppState,
    theme: &crate::core::theme::Theme,
) {
    grid.fill(*area, theme.surface);

    let hint = match state.mode {
        crate::core::mode::Mode::Normal => {
            "tab:switch category  up down:tasks  enter:activate  /:search  ?:help  ctrl c:quit"
        }
        crate::core::mode::Mode::Search => {
            "type to search  up down:navigate  enter:select  esc:cancel"
        }
        crate::core::mode::Mode::Confirm => "y:confirm  n:decline  esc:cancel",
        crate::core::mode::Mode::Help => "esc or ?:close",
        crate::core::mode::Mode::Task => "esc or ctrl c:cancel",
    };
    grid.write_str(area.x + 1, area.y, hint, theme.muted, theme.surface, false);
}
