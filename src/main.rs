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
use mpris::{PlayerFinder, Player, PlaybackStatus};

struct PlayerData {
    finder: PlayerFinder,
    player: Player,
}

struct MetaData {
    status: String,
    title: String,
    artist: String,
    playing: bool,
}


fn get_player() -> Option<PlayerData> {
    let finder = PlayerFinder::new().ok();
    if finder.is_none() {
        return None;
    }
    let player = finder.as_ref().unwrap().find_active().ok();
    if player.is_none() {
        return None
    }
    return Some(PlayerData {
        finder: finder.unwrap(),
        player: player.unwrap(),
    });
}

fn get_metadata() -> Option<MetaData> {
    let playerData = get_player();
    if playerData.is_none() {
        return None;
    }
    let player = playerData.unwrap().player;
    let metadataRaw = player.get_metadata().ok();
    if metadataRaw.is_none() {
        return None;
    }
    let metadata = metadataRaw.unwrap();
    let title = metadata.title().unwrap_or("Unknown title");
    let artist = metadata.artists().unwrap_or((&[]).to_vec());
    let status = player.get_playback_status().ok();
    if status.is_none() {
        return None;
    }
    let state_icon = match status.unwrap() {
            PlaybackStatus::Paused => "󰐊 ",
            PlaybackStatus::Playing => "󰏤 ",
            _ => "",
    };
    let playing = match status.unwrap() {
        PlaybackStatus::Paused => false,
        PlaybackStatus::Playing => true,
        _ => false,
    };

    return Some(MetaData {
        title: title.to_string(),
        artist: artist.join(", "),
        status: state_icon.to_string(),
        playing: playing,
    });


}

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
                let metaData = get_metadata();
                if metaData.is_none() {
                    println!("");
                } else { 
                    let unwrapedData = metaData.unwrap();
                    println!("{}{} - {}", unwrapedData.status, unwrapedData.artist, unwrapedData.title);
                }
                thread::sleep(dont_fry_my_cpu);
            }
        },
        "--hook-waybar" => {
            loop {
                let metaData = get_metadata();
                if metaData.is_none() {
                    println!("");
                } else { 
                    let unwrapedData = metaData.unwrap();
                    println!("{{ \"text\":\"{}{} - {}\" }}", unwrapedData.status, unwrapedData.artist, unwrapedData.title);
                }
                thread::sleep(dont_fry_my_cpu);
            }
        },
        "--current-song" => {
            let metaData = get_metadata();
            if metaData.is_none() {
                println!("");
                return;
            }
            let unwrapedData = metaData.unwrap();
            println!("{}{} - {}", unwrapedData.status, unwrapedData.artist, unwrapedData.title);
        }
        "--play" => {
            let player = get_player().expect("Could not get Player.").player;
            player.play().expect("Could not get Player to resume.");
            println!("Player is resuming playback.");
        },
        "--pause" => {
            let player = get_player().expect("Could not get Player.").player;
            player.pause().expect("Could not get Player to pause.");
            println!("Player is pausing playback.");
        },
        "--next" => {
             let player = get_player().expect("Could not get Player.").player;
             player.next().expect("Could not get Player to skip the current song.");
             println!("Player is skipping the current song.");
        },
        "--previous" => {
             let player = get_player().expect("Could not get Player.").player;
             player.previous().expect("Could not get Player to return to previous song.");
             println!("Player is returning to previous song.");
        },
        "--toggle" => {
            let player = get_player().expect("Could not get Player.").player;
            let isPlaying = get_metadata().expect("Could not get State of Player.").playing;
            if isPlaying {
                player.pause().expect("Could not pause the Player.");
                println!("Paused Player Success.");
            } else {
                player.play().expect("Could not get the Player to resume playing.");
                println!("Player Resume Sucess.");
            }
        },
        _ => {
            print!("{}: unrecognized option '{}'\n", program, args[1]);
            print!("{}", usage);
            print!("{}", description);
            return;
        }
    }
        
}
