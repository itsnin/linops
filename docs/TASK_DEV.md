# writing a task

## task structure

```
src/tasks/<category>/<name>/
  mod.rs       struct + task trait impl
  state.rs     typed state
  ui.rs        rendering to grid
  keys.rs      key handling
  actions.rs   action helpers
  search.rs    search items
  presets/
    mod.rs
    ubuntu.rs
```

register in src/tasks/mod.rs

## rules

- task a never imports from task b
- core never imports from tasks
- tasks import core traits only
- no apt dnf pacman direct calls
- no blocking the ui
- cp437 charset only
- 16 colors only
- keyboard only
- no config required
- esc up down ctrl c handled by dispatcher
