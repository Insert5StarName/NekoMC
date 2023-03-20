/*
NekoMC (New Efficient Kitten-oriented MPRIS Client)
Copyright (C) 2023 Insert5StarName (https://github.com/Insert5StarName)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
#![allow(non_snake_case)]
use std::env;
use std::fs;

use config::Config;
mod cmds;
mod config;
mod playerctl;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let usage = format!("Usage: {} [option]\n", program);
    let description = "NekoMC - New Efficient Kitten-oriented MPRIS Client\n\n\
                       Options:\n\
                       --hook           Print currently playing song info (name, artist, status) in a Loop.\n\
                       --hook-waybar    Print currently playing song info in Waybar module format in a Loop.\n\
                       --current-song   Print currently playing song info but not looped like the first 2 options. \n\
                       --play           Tell the player to resume playing.\n\
                       --pause          Tell the player to pause.\n\
                       --next           Tell the player to play the next song.\n\
                       --previous       Tell the player to play the previous song.\n\
                       --toggle         Toggle between playing and pause.\n
                       --config         Set Config File (Must be 2nd Arguemnt)\n";
    let configData: Config;
    if args.len() != 2 && args.len() != 4 {
        print!("{}", usage);
        print!("{}", description);
        return;
    } else if args.len() == 4 && args[2].as_str() == "--config" {
        let filepath = args[3].as_str();
        if !utils::does_file_exist(filepath) {
            println!("ERROR: Specified Config does not exist.");
            return;
        }
        configData = config::readConfig(filepath);
    } else if args.len() == 4 {
        print!("{}", usage);
        print!("{}", description);
        return;
    } else {
        let combined_please_kill_me_for_doing_this =
            std::env::var("HOME").unwrap_or("$HOME".to_owned()) + "/.config/NekoMC/config.ini";
        let filepath: &str = &&combined_please_kill_me_for_doing_this;
        if utils::does_file_exist(filepath) {
            configData = config::readConfig(filepath)
        } else {
            configData = config::defaultConfig()
        }
    }
    cmds::handleCMDS(args[1].as_str(), configData)
}
