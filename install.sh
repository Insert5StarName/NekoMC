#!/usr/bin/env bash
#NekoMC (New Efficient Kitten-oriented MPRIS Client)
#Copyright (C) 2023 Insert5StarName (https://github.com/Insert5StarName)
#This program is free software: you can redistribute it and/or modify
#it under the terms of the GNU General Public License as published by
#the Free Software Foundation, either version 3 of the License, or
#(at your option) any later version.
#This program is distributed in the hope that it will be useful,
#but WITHOUT ANY WARRANTY; without even the implied warranty of
#MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#GNU General Public License for more details.
#You should have received a copy of the GNU General Public License
#along with this program.  If not, see <http://www.gnu.org/licenses/>.
set -e
if ! command -v cargo > /dev/null || ! command -v dbus-send > /dev/null || ! command -v sudo > /dev/null; then
    echo "Please install dependencies: dbus rust sudo"
    exit 1
fi
echo "Downloading files..."
# Implement CHECKSUMS THIS IS UNSAFE
sudo curl -L https://github.com/Insert5StarName/NekoMC/releases/download/1.0/NekoMC -o "/usr/bin/NekoMC"
sudo chmod +x /usr/bin/NekoMC
echo "Installation successful! Check the Wiki page of the Repo for info on how to use this on Bars like Polybar or Waybar."
