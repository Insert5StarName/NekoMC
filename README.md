# NekoMC (New Efficient Kitten-oriented MPRIS Client)
<p align="center">
    <img src="https://cdn.discordapp.com/attachments/1086458749243363458/1086614668111532152/image.png" style="width: 30%;" alt="bar image"></img>
</p>

a waybar/general component for displaying current song data in bars like waybar and controling players using MPRIS

MPRIS is very widely supported, so this component should work with:
* Firefox and Firefox based Browsers like Librewolf
* Browsers based on Chromium
* Spotify
* mpd (with [mpDris2](https://github.com/eonpatapon/mpDris2))
* VLC ( For People that don't use MPV in 2023 )
* Basically every other Music/Video player on the face of Earth

## Install
Run `sh <(curl -fsSL https://raw.githubusercontent.com/Insert5StarName/NekoMC/main/install.sh)` in your Terminal or

Get the latest release [binary](https://github.com/Insert5StarName/NekoMC/releases/tag/1.0) and move it to `/usr/bin` or your `PATH`
or compile it yourself

## Compile your self
Clone this Repo and Install the needed dependencies (if you use NixOS you can use nix-shell)
* Rust (Cargo, Rustc)
* Dbus
* Pkgconfig

then compile the programm with `cargo build --release` and then copy the file that is located at `./target/release/NekoMC` to `/usr/bin` or your `PATH`


## Usage
When running NekoMC with --hook-waybar it will print out the current song info in a JSON format that Waybar can use.
Add something like this to your waybar `config.json`:
```
"custom/NekoMC": {
          "return-type":"json",
          "exec":"NekoMC --hook-waybar",
          "on-click":"NekoMC --toggle",
          "on-click-right":"NekoMC --next",
          "on-scroll-up":"NekoMC --next",
          "on-scroll-down":"NekoMC --previous",
          "escape" : "true",
},
```


```
Usage: NekoMC [option]
NekoMC - New Efficient Kitten-oriented MPRIS Client

Options:
    --help           Print this help page and exit.
    --version        Print the version and exit.
    --hook           Print currently playing song info (name, artist, status) in a Loop.
    --hook-waybar    Print currently playing song info in Waybar module format in a Loop.
    --current-song   Print currently playing song info but not looped like the first 2 options. 
    --play           Tell the player to resume playing.
    --pause          Tell the player to pause.
    --next           Tell the player to play the next song.
    --previous       Tell the player to play the previous song.
    --toggle         Toggle between playing and pause.
```

## Manual page
```
man NekoMC
```
