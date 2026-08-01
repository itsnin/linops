# architecture

```
                      linops binary runs as user
                                |
                   +------------+------------+
                   |                         |
              core crate                tui crate
           no ui deps no task deps    ratatui crossterm
                   |                         |
         state msg update view       terminal input render runner
         mode search dispatch
         handlers distro
                   |
              grid<cell>
                   |
          +--------+
          |
     tui renderer
     grid to ansi
     via ratatui
          |
     terminal
     or tty1
                                |
                      linops-helper binary runs as root
                      spawned via pkexec or sudo
                      receives action json on stdin
                      streams logs to stdout
```

## principles

1. core has zero ui and zero task dependencies
2. tasks return actions never touch system directly
3. tui never runs as root
4. 16 colors cp437 charset only
5. keyboard only no mouse
6. elm architecture
7. just works philosophy
8. mode scoped input one dispatcher three layers
9. each task fully self contained
10. tui only no gui
