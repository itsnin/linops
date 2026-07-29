sudo apt-get remove --purge -y snapd      # Kill snapd + configs
sudo apt-get autoremove -y                # Clean orphaned deps
sudo apt-mark hold snapd                  # PREVENT REINSTALL
