#!/usr/bin/env bash

printf "Please install dependencys(if missing): dbus pkgconfig cargo"
sleep 1

printf "Downloading file...\n"
#download to $HOME/.cache
version=1.0
downloaction=$HOME/.cache
curl -L https://github.com/Insert5StarName/NekoMC/archive/refs/tags/1.0.tar.gz -o $downloaction/NekoMC.tar.gz
tar xvzf $downloaction/NekoMC
printf "building..."
cd NekoMC-$version
cargo build


# ask user where to put(waybar/polybar)
printf "Where do you want to store the file: "
read -r location
#mv $downloaction/NekoMC.tar.gz "$location"
printf "claiming sudo privliage to install bin to /usr/bin"
sudo mv "NekoMC-$version/target/debug/NekoMC" /usr/bin
printf "if you are using polybar, remenber to put this to your polybar config:\n\n\n"
printf "\n[module/musics]
type = custom/script
exec = $HOME/.config/bspwm/scripts/NekoMC --current-song
tail = true
click-left = $HOME/.config/bspwm/scripts/NekoMC --toggle
click-right = $HOME/.config/bspwm/scripts/NekoMC --next
\n\n\n"
printf "\nif you are using waybar, remenber to put this to your waybar config:\n\n\n"
printf '\n"custom/NekoMC": {
          "return-type":"json",
          "exec":"$HOME/.config/waybar/NekoMC --hook-waybar",
          "on-click":"$HOME/.config/waybar/NekoMC --toggle",
          "on-click-right":"$HOME/.config/waybar/NekoMC --next",
          "on-scroll-up":"$HOME/.config/waybar/NekoMC --next",
          "on-scroll-down":"$HOME/.config/waybar/NekoMC --previous",
          "escape" : "true",
 },\n'

 sudo rm -rf $downloaction/NekoMC
 sudo rm -rf NekoMC-$version

 printf "\n\nIf no errors, then well, NekoMC install sussfully!!!\n"

