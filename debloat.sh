#!/usr/bin/env bash
#
# debloat.sh  —  Pure vanilla GNOME 50 on Ubuntu 26.04 LTS (Server install)
# --------------------------------------------------------------------------
# Minimal, no Ubuntu session / Yaru theme / Ubuntu extensions / Snap.
# Run as:  sudo bash debloat.sh
#
# TARGET:  Ubuntu 26.04 LTS "Resolute Raccoon" Server (amd64) install.
# RESULT:  Vanilla GNOME 50 desktop, ghostty terminal, NetworkManager,
#          no Snap, no Ubuntu skin, no optional GNOME apps. GDM boots the
#          vanilla 'GNOME' session (no 'Ubuntu' session exists).
#
# ============================================================================
# WHY THIS ORDER (GDM enable BEFORE pinning, app removal LAST)
# ----------------------------------------------------------------------------
# The previous version of this script enabled GDM at the very end, AFTER the
# apt pin and autoremove. If anything went wrong during pinning or autoremove
# (and it did — see the ubuntu-wallpapers-resolute bug below), gdm3.service
# would be gone by the time `systemctl enable gdm3` ran, producing:
#     "Failed to enable unit: Unit gdm3.service does not exist"
#
# This version does, in order:
#   0. apt-get update + apt-get upgrade  (bring system to current)
#   1. Install GNOME core + network + terminal
#   2. apt-mark manual EVERYTHING that must survive (so autoremove is safe)
#   3. Enable GDM + graphical.target  ← done EARLY, while gdm3 is fresh
#   4. Install user extras (htop, wget, fonts-noto, gnome-boxes, micro, gnome-shell-extension-manager, Chrome)
#   5. Set breeze-cursor-theme as default cursor (system-wide + GNOME)
#   6. Remove kdump-tools (free 512 MB)
#   7. ONLY THEN remove optional GNOME apps and write apt pins
#   8. autoremove --purge as the very last step
#   9. Install development toolchain (separate, optional, last)
#
# ============================================================================
# THE BUG THAT BROKE THE PREVIOUS SCRIPT (verified, see evidence at bottom)
# ----------------------------------------------------------------------------
# The previous script pinned `ubuntu-wallpapers-resolute` at Pin-Priority: -1
# and tried to `apt remove --purge ubuntu-wallpapers-resolute`. But:
#
#   gdm3  Depends  gnome-shell (>= 50~alpha)
#   gnome-shell  Depends  ubuntu-wallpapers                 <-- Ubuntu patch
#   ubuntu-wallpapers  Depends  ubuntu-wallpapers-resolute  <-- hard dep
#
# Pinning or removing `ubuntu-wallpapers-resolute` cascades through
# ubuntu-wallpapers -> gnome-shell -> gdm3 -> gnome-session, breaking GNOME.
# This is Ubuntu Bug 1894347 (open since 2020, still present in 26.04):
#   https://lists.ubuntu.com/archives/foundations-bugs/2020-September/431929.html
#   "Can't uninstall ubuntu-wallpapers and ubuntu-wallpapers-bionic
#    without gnome-shell"
#
# FIX: Do NOT pin or remove `ubuntu-wallpapers-resolute` (or `ubuntu-wallpapers`
# or `tecla`). Keep them. They are tiny relative to a working desktop.
# (ubuntu-wallpapers-resolute is ~63 MB of wallpaper data — acceptable cost.)
#
# ============================================================================
set -euo pipefail

# Re-exec as root if needed
if [ "$(id -u)" -ne 0 ]; then
  exec sudo "$0" "$@"
fi

export DEBIAN_FRONTEND=noninteractive

echo "==> Updating package lists"
apt-get update

echo "==> Upgrading installed packages"
apt-get upgrade -y

# ---------------------------------------------------------------------------
# 1. Install PURE GNOME core with --no-install-recommends.
#    Skipping recommends is what avoids pulling in `ubuntu-session` (the
#    Ubuntu skin). The vanilla `gnome-session` is a hard dep of gnome-core
#    and WILL be installed — that's the session we want at the GDM login.
# ---------------------------------------------------------------------------
echo "==> Installing gnome-core (vanilla GNOME, no recommends)"
apt-get install -y --no-install-recommends gnome-core

# ---------------------------------------------------------------------------
# 2. Networking: NetworkManager is what upstream GNOME expects.
# ---------------------------------------------------------------------------
echo "==> Installing NetworkManager"
apt-get install -y network-manager

echo "==> Setting netplan renderer to NetworkManager"
apt-get install -y python3-yaml
for f in /etc/netplan/*.yaml; do
  case "$f" in
    *.orig|*curtin*) continue ;;
  esac
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
netplan apply

# ---------------------------------------------------------------------------
# 3. Terminal: ghostty only. We register ghostty as the system terminal
#    NOW, but defer removal of ptyxis to the very end (section 10) so that
#    if anything earlier in the script breaks, we still have a working
#    terminal on the system.
# ---------------------------------------------------------------------------
echo "==> Installing ghostty and registering as default terminal"
apt-get install -y ghostty
update-alternatives --install /usr/bin/x-terminal-emulator x-terminal-emulator /usr/bin/ghostty 50
update-alternatives --set x-terminal-emulator /usr/bin/ghostty

# Tell GNOME's Ctrl+Alt+T keybinding to launch ghostty.
# Note: `org.gnome.desktop.default-applications.terminal` is marked
# "DEPRECATED" in gsettings-desktop-schemas 50.0, but GNOME 50 on
# Ubuntu 26.04 still reads it for the Ctrl+Alt+T keybinding. Verified
# empirically: without this, Ctrl+Alt+T does nothing; with it,
# Ctrl+Alt+T opens ghostty.
# Run as the calling user (not root) so the setting lands in their dconf.
if [ -n "${SUDO_USER:-}" ] && [ "${SUDO_USER}" != "root" ]; then
  sudo -u "$SUDO_USER" gsettings set org.gnome.desktop.default-applications.terminal exec 'ghostty' 2>/dev/null || true
  sudo -u "$SUDO_USER" gsettings set org.gnome.desktop.default-applications.terminal exec-arg '-e' 2>/dev/null || true
fi

# ---------------------------------------------------------------------------
# 4. Protect EVERYTHING that must survive autoremove.
#
#    This list is the union of:
#      (a) GNOME core we explicitly want (gnome-shell, gdm3, etc.)
#      (b) hard deps of gdm3 / gnome-shell / gnome-control-center that the
#          PREVIOUS script forgot — these got purged by autoremove and
#          broke gdm3 (see screenshot):
#              gnome-session-common  (gdm3 hard-dep)
#              gnome-session-bin     (gdm3 hard-dep)
#              gnome-shell-common    (gnome-shell hard-dep)
#              mutter-common         (gnome-control-center hard-dep)
#              libgdm1               (gdm3 hard-dep)
#              gir1.2-gdm-1.0        (gnome-shell + gdm3 hard-dep)
#      (c) hard deps of gnome-shell that we cannot remove but the previous
#          script correctly did NOT pin (kept for clarity):
#              tecla                 (gnome-shell hard-dep)
#              ubuntu-wallpapers     (gnome-shell hard-dep)
#              ubuntu-wallpapers-resolute (ubuntu-wallpapers hard-dep)
#      (d) xdg-user-dirs-gtk — creates Desktop/Documents/Downloads/Music/
#          Pictures/Videos on first GNOME login. KEPT per user request.
#      (e) ubuntu-server metapackage + kernel metapackages — protect the
#          running kernel from autoremove (previous script's autoremove
#          purged linux-image-unsigned-7.0.0-14-generic, see screenshot).
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
#    (This is the user's explicit request: GDM stuff BEFORE the apt pin.)
# ---------------------------------------------------------------------------
echo "==> Enabling GDM and graphical target"
systemctl enable gdm3
systemctl set-default graphical.target
echo "/usr/sbin/gdm3" > /etc/X11/default-display-manager

# ---------------------------------------------------------------------------
# 6. Extras: htop + Google Chrome (comment out the Chrome block if unwanted).
# ---------------------------------------------------------------------------
echo "==> Installing htop, wget, fonts-noto, gnome-boxes, micro, gnome-shell-extension-manager"
apt-get install -y htop wget fonts-noto gnome-boxes micro gnome-shell-extension-manager

echo "==> Installing Google Chrome (direct .deb)"
# Download Chrome's official .deb directly from Google.
# Using `apt-get install ./file.deb` (instead of `dpkg -i`) lets apt resolve
# Chrome's dependencies (e.g., libu2f-udev, fonts-liberation) automatically.
# The .deb's postinst adds Google's signing key + repo to
# /etc/apt/sources.list.d/google-chrome.list, so future `apt upgrade`
# pulls Chrome updates natively.
# Source: https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
wget -q -O /tmp/google-chrome-stable.deb https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
apt-get install -y /tmp/google-chrome-stable.deb || echo "Chrome install failed (continuing)"
rm -f /tmp/google-chrome-stable.deb

# ---------------------------------------------------------------------------
# 6b. Breeze cursor theme (KDE's default cursor) — set as system-wide default.
#
#     Why: GNOME's default Adwaita cursor is white/small; Breeze is larger,
#     has better visibility, and matches what KDE/Plasma users expect.
#
#     IMPLEMENTATION NOTES (verified, see evidence at bottom):
#
#     (1) `apt install breeze-cursor-theme`
#         Installs cursor files to /usr/share/icons/breeze_cursors/cursors/
#         and /usr/share/icons/Breeze_Light/cursors/.
#         Package: breeze-cursor-theme 4:6.6.5-0ubuntu0.1 (resolute-updates/
#         universe). Depends: (none). ~30 MB installed.
#         Source: https://packages.ubuntu.com/resolute/breeze-cursor-theme
#
#     (2) `update-alternatives --install ... x-cursor-theme ...`
#         Registers breeze as an option in the x-cursor-theme alternatives
#         system. The MASTER link is /usr/share/icons/default/index.theme
#         (verified by reading adwaita-icon-theme's postinst, which uses the
#         same master link). breeze-cursor-theme's own postinst does NOT
#         register an alternative (it only runs update-icon-caches), so we
#         must register it manually.
#         We use /usr/share/icons/breeze_cursors/index.theme as the slave
#         path (the only .theme file breeze-cursor-theme ships in 26.04).
#
#         *** IMPORTANT: the OLD path /etc/X11/cursors/breeze_cursors.theme
#             does NOT exist in Ubuntu 26.04's breeze-cursor-theme package.
#             It was removed in version 4:6.2.4-1 (verified by reading the
#             postinst's dpkg-maintscript-helper rm_conffile call, and by
#             fetching the filelist from packages.ubuntu.com/resolute).
#             Older blog posts / Ask Ubuntu answers reference this path —
#             they are OUTDATED (the file still exists in noble 24.04 but
#             was dropped for resolute 26.04). ***
#
#         *** WHY USE `update-alternatives` ON WAYLAND-ONLY GNOME 50? ***
#         GNOME 50 removed the X11 session backend from Mutter/GDM/gnome-shell
#         (verified by multiple sources: XDA, gHacks, The Register, Fedora
#         Project Wiki). There is no "GNOME on Xorg" session at login.
#         HOWEVER, `update-alternatives --set x-cursor-theme` is still
#         useful and correct on Wayland-only GNOME 50, because:
#           (a) `update-alternatives` is a filesystem symlink tool — it
#               repoints /usr/share/icons/default/index.theme to whatever
#               path you specify. It does NOT require an X server to run
#               and does NOT care about display protocols.
#           (b) The "/etc/X11/" in Debian's directory naming convention
#               is a fossil — it's just where Debian historically stored
#               cursor-theme alternative files. The directory name has
#               no runtime significance.
#           (c) XWayland (the X11 compatibility layer) is still bundled
#               in GNOME 50 — X11 apps running inside the Wayland session
#               (older GTK3 apps, Wine, some Electron apps) still read
#               /usr/share/icons/default/index.theme to find the cursor.
#           (d) The Xcursor bitmap format is still what Mutter reads to
#               draw the pointer, regardless of display protocol.
#           (e) The GDM login screen uses its own cursor loading (separate
#               from your dconf user settings) — `update-alternatives` is
#               what configures the GDM cursor.
#         Sources:
#           - https://www.ghacks.net/2025/09/13/gnome-50-releases-with-x11-session-support-removed-and-wayland/
#           - https://fedoraproject.org/wiki/Changes/WaylandOnlyGNOME
#           - https://wiki.debian.org/DebianAlternatives
#
#     (3) `update-alternatives --set x-cursor-theme ...`
#         Selects breeze as the active alternative. This makes breeze the
#         system-wide default cursor for X11 apps that read
#         /usr/share/icons/default/index.theme (e.g., display managers,
#         non-GNOME X11 apps). Priority 50 (lower than Adwaita's 90, but
#         we explicitly --set it so priority doesn't matter).
#
#     (4) `gsettings set org.gnome.desktop.interface cursor-theme 'breeze_cursors'`
#         This is what ACTUALLY changes the cursor inside GNOME (both
#         Wayland and X11 sessions). GNOME's mutter reads this key and
#         loads cursors from /usr/share/icons/breeze_cursors/cursors/.
#         The key is NOT deprecated in GNOME 50 (verified by extracting
#         org.gnome.desktop.interface.gschema.xml from
#         gsettings-desktop-schemas 50.0-1ubuntu2 — default 'Adwaita',
#         no deprecation notice, unlike org.gnome.desktop.default-
#         applications.terminal which IS deprecated).
#         Run as $SUDO_USER so the setting lands in the user's dconf
#         (~/.config/dconf/user), NOT root's.
#
#     ORDER DEPENDENCY:
#       - apt install breeze-cursor-theme: no GNOME dep, can run anytime
#       - update-alternatives: no GNOME dep, just needs dpkg
#       - gsettings set: NEEDS GNOME (gsettings-desktop-schemas + dconf),
#         so MUST run after section 1 (gnome-core install)
#       All three are safe to run here in section 6b because GNOME was
#       installed in section 1.
#
#     TROUBLESHOOTING:
#       - If cursor doesn't change after reboot: log out and back in
#         (mutter reads cursor-theme at session start, not live).
#       - If only some apps show the new cursor: that's a Wayland/X11
#         split — see "Issues worth noting" in the README.
# ---------------------------------------------------------------------------
echo "==> Installing breeze-cursor-theme (KDE cursor as default)"
apt-get install -y breeze-cursor-theme

# Register breeze as an x-cursor-theme alternative.
# Master link /usr/share/icons/default/index.theme is the same one Adwaita
# uses (verified from adwaita-icon-theme postinst). Slave path is the only
# .theme file breeze-cursor-theme ships.
update-alternatives --install /usr/share/icons/default/index.theme x-cursor-theme /usr/share/icons/breeze_cursors/index.theme 50 2>/dev/null || true
update-alternatives --set x-cursor-theme /usr/share/icons/breeze_cursors/index.theme 2>/dev/null || true

# Set the cursor theme in GNOME (per-user dconf setting).
# This is the step that actually changes the cursor inside GNOME.
# Guard: only run if there's a real SUDO_USER (someone ran `sudo bash debloat.sh`
# from a logged-in session). If run from a root shell or cloud-init, this
# skips silently — the user can run the gsettings command manually after
# first login.
if [ -n "${SUDO_USER:-}" ] && [ "${SUDO_USER}" != "root" ]; then
  sudo -u "$SUDO_USER" gsettings set org.gnome.desktop.interface cursor-theme 'breeze_cursors' 2>/dev/null || true
fi

# ---------------------------------------------------------------------------
# 7. Free the ~512 MB the kernel reserves for crash dumps (kdump).
#    Safe to remove on a desktop; we have already protected the running
#    kernel metapackages with apt-mark manual above.
# ---------------------------------------------------------------------------
echo "==> Removing kdump-tools (frees ~512 MB reserved memory)"
apt-get remove -y --purge kdump-tools 2>/dev/null || true
rm -f /etc/default/grub.d/kdump-tools.cfg
update-grub

# ---------------------------------------------------------------------------
# 8. Drop the gnome-core metapackage wrapper.
#    All core pkgs we want are now marked manual (section 4), so dropping
#    the metapkg just makes the optional apps eligible for autoremove.
#    We do this BEFORE removing the optional apps so their removal doesn't
#    cascade back through gnome-core.
# ---------------------------------------------------------------------------
echo "==> Removing gnome-core metapackage (core pkgs are protected by apt-mark manual)"
apt-get remove -y gnome-core 2>/dev/null || true

# ---------------------------------------------------------------------------
# 9. Remove the optional GNOME apps (bloat).
#
#    VERIFIED SAFE-TO-REMOVE (none are hard deps of gnome-shell or gdm3):
#       gnome-calculator, gnome-calendar, gnome-characters, gnome-clocks,
#       gnome-contacts, gnome-disk-utility, gnome-font-viewer, gnome-logs,
#       gnome-maps, gnome-weather, gnome-sushi, gnome-system-monitor,
#       gnome-text-editor, baobab, loupe, papers, showtime, simple-scan,
#       gnome-connections, gnome-user-docs, yelp,
#       orca, gnome-software
#
#    The reverse-depends analysis (from Ubuntu 26.04 resolute Packages.gz)
#    shows each of these is ONLY hard-required-by metapackages that we are
#    also removing (gnome-core) or that aren't installed on Ubuntu Server
#    (cinnamon-desktop-environment, phosh-*, ubuntu-mate-*, etc.).
#
#    *** NOT IN THIS LIST (would break GNOME) ***
#       ubuntu-wallpapers-resolute  (hard dep of ubuntu-wallpapers -> gnome-shell)
#       ubuntu-wallpapers           (hard dep of gnome-shell)
#       tecla                       (hard dep of gnome-shell & gnome-control-center)
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
# ---------------------------------------------------------------------------
echo "==> Removing ptyxis and stray terminals (ghostty is the only terminal)"
apt-get remove -y --purge ptyxis 2>/dev/null || true
apt-get remove -y --purge alacritty xterm gnome-terminal 2>/dev/null || true

# Also make sure snapd is gone (Server doesn't ship it, but ubuntu-server
# Recommends snapd, so it may be present).
echo "==> Removing snapd"
apt-get remove -y --purge snapd 2>/dev/null || true

# ---------------------------------------------------------------------------
# 11. Pin everything we don't want, so apt upgrade can NEVER reinstall it.
#
#     *** CRITICAL: do NOT pin the following (would break GNOME) ***
#        ubuntu-wallpapers-resolute   (hard dep of ubuntu-wallpapers)
#        ubuntu-wallpapers            (hard dep of gnome-shell)
#        tecla                        (hard dep of gnome-shell)
#        vim, vim-common, vim-runtime (hard dep of ubuntu-server)
#
#     `vim-tiny` is safe to pin (it's not a hard dep of anything we keep).
#
#     `ubuntu-session`, `gnome-shell-ubuntu-extensions`,
#     `yaru-theme-gnome-shell` are safe to pin: gdm3 lists them only as
#     one of several alternatives (`ubuntu-session | gnome-session | ...`)
#     and we have `gnome-session` installed, so the alternative is
#     satisfied without them.
#
#     `ptyxis` is pinned even though it's not a hard dep of anything we
#     keep — it's a hard dep of `gnome-core` (which we removed in section
#     8) and a Recommends of `ubuntu-desktop`/`vanilla-gnome-desktop`.
#     Pinning it is defense-in-depth: if the user later does
#     `apt install gnome-core` (to undo), ptyxis will NOT come back.
# ---------------------------------------------------------------------------
echo "==> Writing apt pins (Priority -1 = never install)"
cat > /etc/apt/preferences.d/block-gnome-bloat <<'EOF'
# Pure-GNOME pins: block Ubuntu skin, Snap, removed bloat apps/terminals.
#
# DO NOT add to this list (would break GNOME, verified from Packages.gz):
#   ubuntu-wallpapers-resolute  (hard dep of ubuntu-wallpapers -> gnome-shell)
#   ubuntu-wallpapers           (hard dep of gnome-shell)
#   tecla                       (hard dep of gnome-shell & gnome-control-center)
#   vim, vim-common, vim-runtime  (hard dep of ubuntu-server)
Package: gnome-calculator gnome-calendar gnome-characters gnome-clocks gnome-contacts gnome-disk-utility gnome-font-viewer gnome-logs gnome-maps gnome-weather gnome-sushi gnome-system-monitor gnome-text-editor baobab loupe papers showtime simple-scan gnome-connections gnome-user-docs yelp orca gnome-software snapd ubuntu-session gnome-shell-ubuntu-extensions yaru-theme-gnome-shell yaru-theme-gtk yaru-theme-icon yaru-theme-sound gsettings-ubuntu-schemas alacritty xterm gnome-terminal ptyxis vim-tiny
Pin: release *
Pin-Priority: -1
EOF

# ---------------------------------------------------------------------------
# 12. Clean up orphans. This is now SAFE because everything we want is
#     marked manual in section 4 (including kernel metapackages and
#     gnome-session-common, gnome-session-bin, gnome-shell-common,
#     mutter-common, libgdm1, gir1.2-gdm-1.0).
# ---------------------------------------------------------------------------
echo "==> Autoremoving orphans (core is protected by apt-mark manual)"
apt-get autoremove -y --purge

# ---------------------------------------------------------------------------
# 13. Final sanity: verify gdm3 is still installed.
# ---------------------------------------------------------------------------
if ! systemctl list-unit-files gdm3.service --all 2>/dev/null | grep -q gdm3; then
  echo "!! WARNING: gdm3.service is missing — something went wrong."
  echo "!! Inspect /var/log/apt/history.log and rerun section 4-5."
  exit 1
fi

echo
echo "DONE. Pure vanilla GNOME 50 is installed."
echo "At the GDM login, only the vanilla 'GNOME' session is available"
echo "(no 'Ubuntu' session exists). Reboot now:  sudo reboot"
echo
echo "Optional GNOME apps were removed and pinned. Snap is gone."
echo "Ghostty is the only terminal. NetworkManager manages networking."
echo "Standard home folders (Desktop/Documents/Downloads/Music/Pictures/Videos)"
echo "will be created on first GNOME login by xdg-user-dirs-gtk."

# ---------------------------------------------------------------------------
# 14. OPTIONAL — Development toolchain (Python, C/C++, Rust, Java, Node.js,
#     DB clients, web tooling).
#
#     Placed LAST, after the GDM sanity check and the DONE banner above —
#     deliberately NOT folded into section 6 (extras). This is a large,
#     independent package list, and `set -euo pipefail` means a single
#     bad/unavailable package name here would halt the script wherever it
#     runs. Running it after the desktop conversion is already installed
#     and sanity-checked means a failure here can never take down
#     GNOME/GDM — worst case, only this block needs a rerun.
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

# ============================================================================
# EVIDENCE & REFERENCES
# ----------------------------------------------------------------------------
# 1. Ubuntu Bug 1894347 — "Can't uninstall ubuntu-wallpapers and
#    ubuntu-wallpapers-bionic without gnome-shell" (open since 2020,
#    still present in 26.04 resolute). The exact same dependency chain
#    exists in 26.04: gnome-shell -> ubuntu-wallpapers -> -resolute.
#    https://lists.ubuntu.com/archives/foundations-bugs/2020-September/431929.html
#
# 2. Ubuntu 26.04 Packages.gz (amd64 main) — verified dependency chains:
#    http://archive.ubuntu.com/ubuntu/dists/resolute/main/binary-amd64/Packages.gz
#      gdm3 Depends: gnome-shell (>= 50~alpha)
#      gnome-shell Depends: ubuntu-wallpapers
#      ubuntu-wallpapers Depends: ubuntu-wallpapers-resolute
#      gdm3 Depends: gnome-session-bin (>= 50~alpha), gnome-session-common (>= 50~alpha),
#                    libgdm1 (= 50.0-0ubuntu1), gir1.2-gdm-1.0 (= 50.0-0ubuntu1)
#      gnome-shell Depends: gnome-shell-common (= 50.1-0ubuntu1.1), tecla
#      gnome-control-center Depends: mutter-common
#      ubuntu-server Depends: vim
#
# 3. Ghostty in Ubuntu 26.04 repos (verified by multiple sources):
#    https://discourse.ubuntu.com/t/ghostty-comes-to-ubuntu/80740
#    https://www.omgubuntu.co.uk/2026/04/ghostty-terminal-ubuntu-26-04-apt-install
#    https://github.com/mkasberg/ghostty-ubuntu
#
# 4. gnome-snapshot is GNOME's camera app (replaces Cheese), in GNOME Core:
#    https://www.omgubuntu.co.uk/2024/03/ubuntu-24-04-swaps-cheese-snapshot-webcam-app
#    https://discourse.ubuntu.com/t/cheese-discontinued-on-26-04-but-not-on-24-04-why/82002
#
# 5. xdg-user-dirs-gtk creates Desktop/Documents/Downloads/Music/Pictures/Videos
#    on first GNOME login (verified by ArchWiki, freedesktop.org, Debian):
#    https://wiki.archlinux.org/title/XDG_user_directories
#    https://www.freedesktop.org/wiki/Software/xdg-user-dirs
#    https://packages.debian.org/sid/xdg-user-dirs-gtk
#
# 6. apt autoremove protects the running kernel via
#    /etc/apt/apt.conf.d/01autoremove (kernel metapackages still need to be
#    manual to ensure future kernel upgrades install):
#    https://askubuntu.com/questions/563483/why-doesnt-apt-get-autoremove-remove-my-old-kernels
#
# 7. breeze-cursor-theme package (Ubuntu 26.04 resolute, universe):
#    https://packages.ubuntu.com/resolute/breeze-cursor-theme
#    Version 4:6.6.5-0ubuntu0.1 — Depends: (none), ~30 MB installed.
#    Filelist (verified — NO /etc/X11/cursors/ files in resolute):
#    https://packages.ubuntu.com/resolute/all/breeze-cursor-theme/filelist
#    Only ships: /usr/share/icons/breeze_cursors/index.theme
#                /usr/share/icons/Breeze_Light/index.theme
#    The OLD /etc/X11/cursors/breeze_cursors.theme was removed in
#    breeze 4:6.2.4-1 (per postinst dpkg-maintscript-helper rm_conffile).
#
# 8. adwaita-icon-theme postinst registers x-cursor-theme alternative:
#      update-alternatives --install /usr/share/icons/default/index.theme \
#        x-cursor-theme /usr/share/icons/Adwaita/cursor.theme 90
#    Master link: /usr/share/icons/default/index.theme
#    breeze-cursor-theme's postinst does NOT register an alternative
#    (only runs update-icon-caches), so we register it manually in 6b.
#
# 9. GNOME cursor-theme gsettings key (NOT deprecated in GNOME 50):
#    Schema: org.gnome.desktop.interface
#    Key: cursor-theme (type s, default 'Adwaita')
#    File: /usr/share/glib-2.0/schemas/org.gnome.desktop.interface.gschema.xml
#    Package: gsettings-desktop-schemas 50.0-1ubuntu2
#    Mutter reads this key to load cursors from
#    /usr/share/icons/<theme>/cursors/ on both Wayland and X11 sessions.
#
# 10. Ptyxis dependency analysis (why it's safe to pin):
#     Hard dep of: edubuntu-desktop, edubuntu-desktop-minimal, gnome-core,
#                  gnome-flashback-meta
#     Recommends of: ubuntu-desktop, ubuntu-desktop-minimal,
#                     vanilla-gnome-desktop
#     We removed gnome-core (section 8), so nothing pulls ptyxis now.
#     Pinning is defense-in-depth against future `apt install gnome-core`.
# ============================================================================
