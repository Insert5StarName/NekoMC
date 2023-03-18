#!/usr/bin/env bash

printf "Downloading file...\n"
#download to $HOME/.cache
downloaction=$HOME/.cache
curl -L https://github.com/Insert5StarName/NekoMC/archive/refs/tags/1.0.tar.gz -o $downloaction/NekoMC.tar.gz


# ask user where to put(waybar/polybar)
printf "Where do you want to store the file: "
read -r location
cp $downloaction/NekoMC.tar.gz "$location" && rm -rf $downlocation/NekoMC.tar.gz
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

