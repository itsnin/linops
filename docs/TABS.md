# adding a task

see ../scripts/LLM.md for the full binding rules and reasoning this
file is the short version

## structure

    core/tabs/<category>/<task_name>/<task_name>.sh

that is the whole task one bash script one folder named after it no
other files belong in a task folder

## rules

- bash specifically not a portable posix sh every real linux install
  has bash even when it is not the login shell
- guard for idempotency check if the thing is already done or
  already absent before acting print a result either way see
  system_management/snap_debloat/snapd.sh
- the script decides its own logic internally there is no rust side
  state mirroring what the script's own branches already decide
- register the task in the category's tab_data.toml under
  core/tabs/<category>/

## rejected approaches do not reintroduce

- a rust struct per task with its own state ui keys files this was
  built once and deleted see LLM.md for why
- a written apt preferences pin file use apt-mark hold instead
- building multi distro branching before it is needed ubuntu only
  until told otherwise
