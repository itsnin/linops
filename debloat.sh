#!/usr/bin/env bash
#
# debloat.sh — Pure vanilla GNOME 50 on Ubuntu 26.04 LTS (Server install)
# --------------------------------------------------------------------------
# Minimal: no Ubuntu session / Yaru theme / Ubuntu extensions / Snap.
# Run as:  sudo ./debloat.sh   (or: ./debloat.sh — it self-elevates)
#
# TARGET:  Ubuntu 26.04 LTS "Resolute Raccoon" Server (amd64) install.
# RESULT:  Vanilla GNOME 50 desktop, ghostty terminal, NetworkManager,
#          breeze cursor, no Snap, no Ubuntu skin, no optional GNOME apps.
#          GDM boots the vanilla 'GNOME' session (no 'Ubuntu' session exists).
#
# ============================================================================
# SCRIPT FLOW (14 sections — numbers match the section headers below)
# ----------------------------------------------------------------------------
#   0. apt-get update + apt-get upgrade      (bring system to current)
#   1. Install gnome-core (--no-install-recommends)
#   2. Install NetworkManager + switch netplan renderer
#   3. Install ghostty + register as default terminal + gsettings for Ctrl+Alt+T
#   4. apt-mark manual EVERYTHING that must survive autoremove
#   5. Enable gdm3 + set graphical.target    ← EARLY, before any pinning
#   6. Install extras + Chrome + set breeze cursor as default
#   7. Remove kdump-tools (free ~512 MB)
#   8. Remove gnome-core metapackage
#   9. Remove optional GNOME apps (23 packages)
#  10. Remove ptyxis + snapd
#  11. Write apt pins (Priority -1 = never install)
#  12. apt autoremove --purge
#  13. Sanity check: gdm3.service exists?
#  14. Install development toolchain (separate, last)
#
# ============================================================================
# WHY GDM IS ENABLED EARLY (section 5, before pinning)
# ----------------------------------------------------------------------------
# If pinning or autoremove fails and purges gdm3, we want gdm3.service to
# already be registered so the system can still boot to a graphical target.
# Doing it the other way around produces:
#     "Failed to enable unit: Unit gdm3.service does not exist"
#
# ============================================================================
# THE ubuntu-wallpapers-resolute TRAP — Ubuntu Bug 1894347 (open since 2020)
# ----------------------------------------------------------------------------
# Do NOT pin or remove these — they look like bloat but are hard deps:
#
#   gdm3  Depends  gnome-shell (>= 50~alpha)
#   gnome-shell  Depends  ubuntu-wallpapers                 ← Ubuntu patch
#   ubuntu-wallpapers  Depends  ubuntu-wallpapers-resolute  ← hard dep
#   gnome-shell  Depends  tecla                             ← hard dep
#
# Pinning/removing any of these cascades through gnome-shell → gdm3 →
# gnome-session and breaks the desktop.
# Ref: https://lists.ubuntu.com/archives/foundations-bugs/2020-September/431929.html
#
# ============================================================================
set -euo pipefail

# Re-exec as root if not already root (so `./debloat.sh` without sudo works).
if [ "$(id -u)" -ne 0 ]; then
  exec sudo "$0" "$@"
fi

# Suppress interactive prompts (e.g., "Restart services automatically? [y/N]").
export DEBIAN_FRONTEND=noninteractive

# ---------------------------------------------------------------------------
# 0. Update + upgrade — bring the system to current before doing anything.
#    If Ubuntu ships a broken package, that's Ubuntu's problem; we start
#    from a known-good baseline.
# ---------------------------------------------------------------------------
echo "==> Updating package lists"
apt-get update

echo "==> Upgrading installed packages"
apt-get upgrade -y

# ---------------------------------------------------------------------------
# 1. Install PURE GNOME core with --no-install-recommends.
#    Skipping recommends avoids pulling in `ubuntu-session` (the Ubuntu skin).
#    The vanilla `gnome-session` is a hard dep of gnome-core and WILL be
#    installed — that's the session we want at the GDM login.
#    Ref: https://packages.ubuntu.com/resolute/gnome-core
# ---------------------------------------------------------------------------
echo "==> Installing gnome-core (vanilla GNOME, no recommends)"
apt-get install -y --no-install-recommends gnome-core

# ---------------------------------------------------------------------------
# 2. Networking: NetworkManager is what upstream GNOME expects.
#    Ubuntu Server defaults to systemd-networkd via netplan; GNOME's
#    network panel only works with NetworkManager, so we switch the
#    netplan renderer and apply.
# ---------------------------------------------------------------------------
echo "==> Installing NetworkManager"
apt-get install -y network-manager

echo "==> Setting netplan renderer to NetworkManager"
# python3-yaml is needed for the inline Python script below to edit YAML.
apt-get install -y python3-yaml

# Loop over all netplan configs, skipping backups (*.orig) and curtinstage
# files (left over from the installer — not meant to be edited).
for f in /etc/netplan/*.yaml; do
  case "$f" in
    *.orig|*curtin*) continue ;;
  esac
  # Inline Python: parse YAML, set renderer: NetworkManager, write back.
  # Using PyYAML (not sed) so we don't corrupt the file structure.
  python3 - "$f" <<'PY'
import sys, yaml
f = sys.argv[1]
try:
    with open(f) as fh:
        data = yaml.safe_load(fh)
except Exception as e:
    print("skip", f, e); sys.exit(0)
if isinstance(data, dict) and isinstance(data.get('network'), dict):
    data['network']['renderer'] = 'NetworkManager'
    with open(f, 'w') as fh:
        yaml.safe_dump(data, fh, default_flow_style=False, sort_keys=False)
    print("updated", f)
PY
done

# Apply the new netplan config (may briefly drop network if on netplan).
netplan apply

# ---------------------------------------------------------------------------
# 3. Terminal: ghostty only. Registered NOW but ptyxis removal is deferred
#    to section 10 so that if anything earlier breaks, we still have a
#    working terminal on the system.
#    Ghostty is in Ubuntu 26.04 repos (verified by multiple sources):
#      - https://discourse.ubuntu.com/t/ghostty-comes-to-ubuntu/80740
#      - https://www.omgubuntu.co.uk/2026/04/ghostty-terminal-ubuntu-26-04-apt-install
#      - https://github.com/mkasberg/ghostty-ubuntu
# ---------------------------------------------------------------------------
echo "==> Installing ghostty and registering as default terminal"
apt-get install -y ghostty

# Register ghostty as an x-terminal-emulator alternative (priority 50).
update-alternatives --install /usr/bin/x-terminal-emulator x-terminal-emulator /usr/bin/ghostty 50
# Select ghostty as the active terminal (so `x-terminal-emulator` → ghostty).
update-alternatives --set x-terminal-emulator /usr/bin/ghostty

# Tell GNOME's Ctrl+Alt+T keybinding to launch ghostty.
# This gsettings key is marked "deprecated" in GNOME 50 but still read for
# Ctrl+Alt+T (verified empirically). Run as $SUDO_USER, not root, so the
# setting lands in the user's dconf (~/.config/dconf/user), not root's.
# Guard: skip silently if no SUDO_USER (root shell / cloud-init) — user
# can run the gsettings command manually after first login.
if [ -n "${SUDO_USER:-}" ] && [ "${SUDO_USER}" != "root" ]; then
  sudo -u "$SUDO_USER" gsettings set org.gnome.desktop.default-applications.terminal exec 'ghostty' 2>/dev/null || true
  sudo -u "$SUDO_USER" gsettings set org.gnome.desktop.default-applications.terminal exec-arg '-e' 2>/dev/null || true
fi

# ---------------------------------------------------------------------------
# 4. Protect EVERYTHING that must survive autoremove.
#    Without this, `apt autoremove` (section 12) would purge hard deps
#    of gdm3/gnome-shell and break the desktop.
#
#    (a) GNOME core we explicitly want:
#          gnome-shell, gdm3, gnome-control-center, gnome-session, nautilus,
#          gnome-settings-daemon, gnome-keyring, gnome-menus, gnome-backgrounds,
#          gsettings-desktop-schemas, adwaita-icon-theme, gnome-snapshot,
#          gnome-bluetooth-sendto, ghostty, network-manager, pipewire-audio,
#          xdg-desktop-portal-gnome, libpam-gnome-keyring, gnome-online-accounts
#
#    (b) Hard deps of gdm3 / gnome-shell / gnome-control-center that
#        autoremove would purge (verified from Packages.gz):
#          gnome-session-common   (gdm3 hard-dep, >= 50~alpha)
#          gnome-session-bin      (gdm3 hard-dep, >= 50~alpha)
#          gnome-shell-common     (gnome-shell hard-dep, = 50.1-0ubuntu1.1)
#          mutter-common          (gnome-control-center hard-dep)
#          libgdm1                (gdm3 hard-dep, = 50.0-0ubuntu1)
#          gir1.2-gdm-1.0         (gnome-shell + gdm3 hard-dep, = 50.0-0ubuntu1)
#        Ref: http://archive.ubuntu.com/ubuntu/dists/resolute/main/binary-amd64/Packages.gz
#
#    (c) Hard deps of gnome-shell that we cannot remove or pin:
#          tecla                      (gnome-shell hard-dep)
#          ubuntu-wallpapers          (gnome-shell hard-dep)
#          ubuntu-wallpapers-resolute (ubuntu-wallpapers hard-dep)
#
#    (d) xdg-user-dirs-gtk — creates Desktop/Documents/Downloads/Music/
#        Pictures/Videos on first GNOME login.
#        Ref: https://wiki.archlinux.org/title/XDG_user_directories
#
#    (e) ubuntu-server + kernel metapackages — protect the running kernel
#        from autoremove (kernel metapackages must be manual so future
#        kernel upgrades install).
#        Ref: https://askubuntu.com/questions/563483/why-doesnt-apt-get-autoremove-remove-my-old-kernels
# ---------------------------------------------------------------------------
echo "==> Marking critical packages as manually installed (protects autoremove)"
apt-mark manual \
  gnome-shell gdm3 gnome-control-center gnome-session nautilus \
  gnome-settings-daemon gnome-keyring gnome-menus gnome-backgrounds \
  gsettings-desktop-schemas adwaita-icon-theme gnome-snapshot gnome-bluetooth-sendto ghostty \
  network-manager pipewire-audio xdg-desktop-portal-gnome \
  libpam-gnome-keyring gnome-online-accounts xdg-user-dirs-gtk \
  gnome-session-common gnome-session-bin gnome-shell-common mutter-common \
  libgdm1 gir1.2-gdm-1.0 \
  tecla ubuntu-wallpapers ubuntu-wallpapers-resolute \
  ubuntu-server linux-image-generic linux-generic

# ---------------------------------------------------------------------------
# 5. Make GDM the display manager and boot into the graphical target.
#    DONE EARLY — while gdm3 is freshly installed and we KNOW it exists.
#    If we did this after pinning/autoremove, gdm3 might already be gone.
# ---------------------------------------------------------------------------
echo "==> Enabling GDM and graphical target"
systemctl enable gdm3                          # enable the GDM systemd unit
systemctl set-default graphical.target         # boot to GUI, not multi-user
echo "/usr/sbin/gdm3" > /etc/X11/default-display-manager  # tell dpkg GDM is the DM

# ---------------------------------------------------------------------------
# 6. Extras: htop, wget, fonts-noto, gnome-boxes, micro,
#    gnome-shell-extension-manager, breeze-cursor-theme, Google Chrome.
# ---------------------------------------------------------------------------
echo "==> Installing extras (htop, wget, fonts-noto, gnome-boxes, micro, gnome-shell-extension-manager, breeze-cursor-theme)"
apt-get install -y htop wget fonts-noto gnome-boxes micro gnome-shell-extension-manager breeze-cursor-theme

echo "==> Installing Google Chrome (direct .deb)"
# apt-get install (not dpkg -i) lets apt resolve Chrome's deps automatically.
# The .deb's postinst adds Google's signing key + apt repo, so future
# `apt upgrade` pulls Chrome updates natively.
wget -q -O /tmp/google-chrome-stable.deb https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
apt-get install -y /tmp/google-chrome-stable.deb || echo "Chrome install failed (continuing)"
rm -f /tmp/google-chrome-stable.deb

echo "==> Setting breeze-cursor-theme as default cursor"
# breeze-cursor-theme's postinst does NOT register an x-cursor-theme
# alternative (only runs update-icon-caches), so we do it manually.
# Master link is /usr/share/icons/default/index.theme (same one Adwaita
# uses — verified from adwaita-icon-theme postinst: it runs
# `update-alternatives --install /usr/share/icons/default/index.theme
# x-cursor-theme /usr/share/icons/Adwaita/cursor.theme 90`).
# Path is /usr/share/icons/breeze_cursors/index.theme — the OLD
# /etc/X11/cursors/breeze_cursors.theme was removed in breeze 4:6.2.4-1
# and does NOT exist in Ubuntu 26.04 resolute.
# Ref: https://packages.ubuntu.com/resolute/all/breeze-cursor-theme/filelist
#
# The --install is suppressed (2>/dev/null) but surfaces a warning on
# failure — this package has changed its file layout once before, so if
# it shifts again in a future point release, we want a signal rather
# than silently keeping Adwaita.
update-alternatives --install /usr/share/icons/default/index.theme x-cursor-theme /usr/share/icons/breeze_cursors/index.theme 50 2>/dev/null \
  || echo "!! breeze cursor alternative failed to register — check the path still exists"
# --set overrides priority-based auto-selection, so breeze (50) wins
# over Adwaita (90) because we explicitly pick it.
update-alternatives --set x-cursor-theme /usr/share/icons/breeze_cursors/index.theme 2>/dev/null || true
# gsettings is what mutter actually reads at session start (per-user dconf,
# run as $SUDO_USER not root). The key `cursor-theme` in
# org.gnome.desktop.interface is NOT deprecated in GNOME 50.
if [ -n "${SUDO_USER:-}" ] && [ "${SUDO_USER}" != "root" ]; then
  sudo -u "$SUDO_USER" gsettings set org.gnome.desktop.interface cursor-theme 'breeze_cursors' 2>/dev/null || true
fi

# ---------------------------------------------------------------------------
# 7. Free the ~512 MB the kernel reserves for crash dumps (kdump).
#    Safe to remove on a desktop; we already protected the running kernel
#    metapackages with apt-mark manual in section 4.
# ---------------------------------------------------------------------------
echo "==> Removing kdump-tools (frees ~512 MB reserved memory)"
apt-get remove -y --purge kdump-tools 2>/dev/null || true
rm -f /etc/default/grub.d/kdump-tools.cfg       # remove kdump's GRUB config snippet
update-grub                                      # regenerate GRUB config without kdump

# ---------------------------------------------------------------------------
# 8. Drop the gnome-core metapackage wrapper.
#    All core pkgs we want are now marked manual (section 4), so dropping
#    the metapkg just makes the optional apps eligible for autoremove.
#    Done BEFORE removing the optional apps so their removal doesn't
#    cascade back through gnome-core's deps.
# ---------------------------------------------------------------------------
echo "==> Removing gnome-core metapackage (core pkgs are protected by apt-mark manual)"
apt-get remove -y gnome-core 2>/dev/null || true

# ---------------------------------------------------------------------------
# 9. Remove the optional GNOME apps (bloat).
#
#    All 23 packages verified safe-to-remove against Ubuntu 26.04 resolute
#    Packages.gz — none is a hard dep of gnome-shell, gdm3,
#    gnome-control-center, gnome-session, nautilus, or ubuntu-server.
#    Each is only hard-required-by metapackages we're also removing
#    (gnome-core) or that aren't installed on Ubuntu Server
#    (cinnamon-desktop-environment, phosh-*, ubuntu-mate-*, etc.).
#
#    *** NOT IN THIS LIST (would break GNOME — see header) ***
#       ubuntu-wallpapers-resolute, ubuntu-wallpapers, tecla
# ---------------------------------------------------------------------------
echo "==> Removing optional GNOME apps (bloat)"
apt-get remove -y --purge \
  gnome-calculator gnome-calendar gnome-characters gnome-clocks gnome-contacts \
  gnome-disk-utility gnome-font-viewer gnome-logs gnome-maps gnome-weather \
  gnome-sushi gnome-system-monitor gnome-text-editor baobab loupe papers \
  showtime simple-scan gnome-connections gnome-user-docs \
  yelp orca gnome-software

# ---------------------------------------------------------------------------
# 10. Remove the default terminal (ptyxis) — we already registered ghostty.
#     Also remove any stray terminal that may have sneaked in.
#     (All `2>/dev/null || true` because these may not be installed.)
# ---------------------------------------------------------------------------
echo "==> Removing ptyxis and stray terminals (ghostty is the only terminal)"
apt-get remove -y --purge ptyxis 2>/dev/null || true
apt-get remove -y --purge alacritty xterm gnome-terminal 2>/dev/null || true

# Remove snapd — Server doesn't ship it, but ubuntu-server Recommends it.
echo "==> Removing snapd"
apt-get remove -y --purge snapd 2>/dev/null || true

# ---------------------------------------------------------------------------
# 11. Pin everything we don't want, so apt upgrade can NEVER reinstall it.
#
#     *** DO NOT PIN (would break GNOME — see header) ***
#        ubuntu-wallpapers-resolute   (hard dep of ubuntu-wallpapers)
#        ubuntu-wallpapers            (hard dep of gnome-shell)
#        tecla                        (hard dep of gnome-shell)
#        vim, vim-common, vim-runtime (hard dep of ubuntu-server)
#
#     Safe to pin — these are NOT hard deps of anything we keep:
#        vim-tiny                        (not a hard dep of anything)
#        ubuntu-session                  (alt in gdm3's deps: ubuntu-session | gnome-session | ...)
#        gnome-shell-ubuntu-extensions   (alt in gdm3's deps)
#        yaru-theme-gnome-shell          (Recommends of gnome-shell-common, not hard)
#        yaru-theme-gtk/icon/sound       (Recommends, not hard)
#        gsettings-ubuntu-schemas        (not a hard dep of anything we keep)
#        ptyxis                          (hard dep of gnome-core, which we removed;
#                                         pinned as defense-in-depth against
#                                         future `apt install gnome-core`)
#        alacritty, xterm, gnome-terminal (not installed, not deps)
#        snapd                           (Recommends of ubuntu-server, not hard)
#        all 23 GNOME apps from section 9 (verified safe — see above)
# ---------------------------------------------------------------------------
echo "==> Writing apt pins (Priority -1 = never install)"
cat > /etc/apt/preferences.d/block-gnome-bloat <<'EOF'
# Pure-GNOME pins: block Ubuntu skin, Snap, removed bloat apps/terminals.
# DO NOT add to this list (would break GNOME, verified from Packages.gz):
#   ubuntu-wallpapers-resolute, ubuntu-wallpapers, tecla, vim/vim-common/vim-runtime
Package: gnome-calculator gnome-calendar gnome-characters gnome-clocks gnome-contacts gnome-disk-utility gnome-font-viewer gnome-logs gnome-maps gnome-weather gnome-sushi gnome-system-monitor gnome-text-editor baobab loupe papers showtime simple-scan gnome-connections gnome-user-docs yelp orca gnome-software snapd ubuntu-session gnome-shell-ubuntu-extensions yaru-theme-gnome-shell yaru-theme-gtk yaru-theme-icon yaru-theme-sound gsettings-ubuntu-schemas alacritty xterm gnome-terminal ptyxis vim-tiny
Pin: release *
Pin-Priority: -1
EOF

# ---------------------------------------------------------------------------
# 12. Clean up orphans. SAFE because everything we want is marked manual
#     in section 4 (kernel metapackages, gnome-session-common/bin,
#     gnome-shell-common, mutter-common, libgdm1, gir1.2-gdm-1.0, etc.).
# ---------------------------------------------------------------------------
echo "==> Autoremoving orphans (core is protected by apt-mark manual)"
apt-get autoremove -y --purge

# ---------------------------------------------------------------------------
# 13. Final sanity: verify gdm3 is still installed. If the pin or autoremove
#     broke something, this is where we catch it before the user reboots.
# ---------------------------------------------------------------------------
if ! systemctl list-unit-files gdm3.service --all 2>/dev/null | grep -q gdm3; then
  echo "!! WARNING: gdm3.service is missing — something went wrong."
  echo "!! Inspect /var/log/apt/history.log and rerun sections 4-5."
  exit 1
fi

echo
echo "DONE. Pure vanilla GNOME 50 is installed."
echo "At the GDM login, only the vanilla 'GNOME' session is available"
echo "(no 'Ubuntu' session exists). Reboot now:  sudo reboot"
echo
echo "Optional GNOME apps were removed and pinned. Snap is gone."
echo "Ghostty is the only terminal. NetworkManager manages networking."
echo "Breeze cursor is the default. Standard home folders"
echo "(Desktop/Documents/Downloads/Music/Pictures/Videos) will be created"
echo "on first GNOME login by xdg-user-dirs-gtk."

# ---------------------------------------------------------------------------
# 14. Development toolchain (Python, C/C++, Rust, Java, Node.js, DB clients,
#     web tooling).
#
#     Placed LAST, after the GDM sanity check and the DONE banner —
#     deliberately NOT folded into section 6. This is a large, independent
#     package list, and `set -euo pipefail` means a single bad/unavailable
#     package name here would halt the script. Running it after the desktop
#     conversion is already installed and sanity-checked means a failure
#     here can never take down GNOME/GDM — worst case, only this block
#     needs a rerun.
# ---------------------------------------------------------------------------
echo "==> Installing development toolchain"
apt-get install -y \
  python3 python3-pip python3-venv python3-dev python3-full \
  libssl-dev libffi-dev zlib1g-dev libbz2-dev liblzma-dev libreadline-dev libsqlite3-dev libncurses-dev \
  build-essential gcc g++ clang clangd clang-format clang-tidy make cmake ninja-build gdb pkg-config valgrind llvm \
  rustc cargo rustfmt rust-clippy \
  default-jdk maven \
  nodejs \
  sqlite3 postgresql-client mariadb-client pgcli mycli \
  tidy html-xml-utils sassc \
  ca-certificates gnupg
echo "==> Development toolchain installed"
