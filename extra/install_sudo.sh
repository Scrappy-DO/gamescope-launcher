#!/usr/bin/bash

sudo cp icon.svg /usr/share/pixmaps/gamescope-launcher.svg
cp gamescope-launcher.desktop ~/.local/share/applications/
sudo cp ../target/release/gamescope_launcher /usr/local/bin
