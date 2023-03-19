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
use std::time;
use std::thread;
use mpris::PlaybackStatus;
mod utils;
mod playerctl;

fn main() {
    let dont_fry_my_cpu = time::Duration::from_millis(500);    
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
                       --toggle         Toggle between playing and pause.\n";

    if args.len() != 2 {
        print!("{}", usage);
        print!("{}", description);
        return;
    }
    match args[1].as_str() {
        "--hook" => {
            loop {
                let player = utils::get_player();
                if player.is_none() {
                    print!("");
                }
                let metaData = utils::get_metadata(player.unwrap());
                if metaData.is_none() {
                    println!("");
                } else { 
                    let unwrapedData = metaData.unwrap();
                    let state_icon = match unwrapedData.status{
                        PlaybackStatus::Paused => "󰐊 ",
                        PlaybackStatus::Playing => "󰏤 ",
                        _ => "",
                    };
                    println!("{}{} - {}", state_icon, unwrapedData.artist, unwrapedData.title);
                }
                thread::sleep(dont_fry_my_cpu);
            }
        },
        "--hook-waybar" => {
            loop {
                let player = utils::get_player();
                if player.is_none() {
                    print!("");
                }
                let metaData = utils::get_metadata(player.unwrap());
                if metaData.is_none() {
                    println!("");
                } else { 
                    let unwrapedData = metaData.unwrap();
                    let state_icon = match unwrapedData.status{
                        PlaybackStatus::Paused => "󰐊 ",
                        PlaybackStatus::Playing => "󰏤 ",
                        _ => "",
                    };
                    println!("{{ \"text\":\"{}{} - {}\" }}", state_icon, unwrapedData.artist, unwrapedData.title);
                }
                thread::sleep(dont_fry_my_cpu);
            }
        },
        "--current-song" => {
            let player = utils::get_player();
                if player.is_none() {
                    print!("");
                }
            let metaData = utils::get_metadata(player.unwrap());
            if metaData.is_none() {
                println!("");
                return;
            }
            let unwrapedData = metaData.unwrap();
            let state_icon = match unwrapedData.status{
                PlaybackStatus::Paused => "󰐊 ",
                PlaybackStatus::Playing => "󰏤 ",
                _ => "",
            };
            println!("{}{} - {}", state_icon, unwrapedData.artist, unwrapedData.title);
        }
        "--play" => playerctl::play(),
        "--pause" => playerctl::pause(),
        "--next" => playerctl::next(),
        "--previous" => playerctl::previous(),
        "--toggle" => playerctl::toggle(),
        _ => {
            print!("{}: unrecognized option '{}'\n", program, args[1]);
            print!("{}", usage);
            print!("{}", description);
            return;
        }
    }
        
}
