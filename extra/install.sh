#!/usr/bin/bash

mkdir -p /opt/gamescope-laucher
cp icon.svg /opt/gamescope-laucher
cp gamescope-laucher.desktop ~/.local/share/applications/
cp ../target/release/gamescope_launcher /opt/gamescope-laucher
