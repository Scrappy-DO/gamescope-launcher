#!/usr/bin/bash

cp icon.svg /usr/share/pixmaps/gamescope-launcher.svg
cp gamescope-launcher.desktop ~/.local/share/applications/
cp ../target/release/gamescope_launcher /usr/local/bin
