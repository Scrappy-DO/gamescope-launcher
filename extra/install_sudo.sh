#!/usr/bin/bash

sudo mkdir -p /opt/gamescope-launcher
sudo cp icon.svg /usr/share/pixmaps/gamescope-launcher.svg
cp gamescope-launcher.desktop ~/.local/share/applications/
sudo cp ../target/release/gamescope_launcher /opt/gamescope-launcher
