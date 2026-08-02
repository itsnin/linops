# distro support

no per distro rust adapter exists anymore see scripts/LLM.md for
why support for a distro means the task's own bash script branches
on it internally the same way linutil's real scripts do (case
"$PACKAGER" in ... see remove-snaps.sh for the reference shape) or
for now while linops is ubuntu only just calls apt directly with a
comment noting where a future distro branch would go

## tier 1 v1

| distro | pkg mgr | init |
|---|---|---|
| ubuntu | apt | systemd |

## tier 2 later

- debian fedora arch

## tier 3 later

- opensuse alpine void nixos

do not build the tier 2 3 branching until it is actually needed see
LLM.md's explicit note against premature multi distro branching
