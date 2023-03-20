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
use crate::config;
use crate::config::Config;
use crate::playerctl;
use crate::playerctl::play;
use crate::utils;
use mpris::PlaybackStatus;
use std::sync::mpsc;
use std::thread;
use std::time;

pub fn handleCMDS(cmd: &str, config: Config) {
    let dont_fry_my_cpu = time::Duration::from_millis(config.general.dont_fry_my_cpu);
    let playicon = config.hook_mode.play_icon.replace("\"", "");
    let pauseicon = config.hook_mode.paused_icon.replace("\"", "");
    let stopicon = config.hook_mode.stopped_icon.replace("\"", "");
    let format = config.hook_mode.format.replace("\"", "");
    let playerinactive = config.hook_mode.format_inactive.replace("\"", "");
    let playerinactiveT2 = playerinactive.clone();
    let (playerInfo, scrolltext) = mpsc::channel();
    let t1 = thread::spawn(move || {
        let mut oldStr = String::from(playerinactive.clone());
        let mut oldIcon = String::from(stopicon.clone());
        loop {
        let player = utils::get_player();
        if player.is_none() {
            if oldStr != playerinactive {
                playerInfo.send((stopicon.clone(), playerinactive.clone())).ok();
                oldStr = playerinactive.clone();
            }
            thread::sleep(dont_fry_my_cpu);
            continue;
        }
        let metaData = utils::get_metadata(player.unwrap());
        if metaData.is_none() {
            if oldStr != playerinactive {
            playerInfo.send((stopicon.clone(), playerinactive.clone())).ok();
            oldStr = playerinactive.clone();
            }
            thread::sleep(dont_fry_my_cpu);
            continue;
        }
        let unwrapedData = metaData.unwrap();
        let icon = match unwrapedData.status {
            PlaybackStatus::Paused => playicon.clone().to_owned(),
            PlaybackStatus::Playing => pauseicon.clone().to_owned(),
            PlaybackStatus::Stopped => stopicon.clone().to_owned(),
        };
        let sendString = format
            .clone()
            .replace("[artist]", &unwrapedData.artist)
            .replace("[title]", &unwrapedData.title);
        if oldStr != sendString || oldIcon != icon {
            playerInfo.send((icon.clone(), sendString.clone())).ok();
            oldStr = sendString;
            oldIcon = icon;
        }
        thread::sleep(dont_fry_my_cpu);
    }
    });

    match cmd {
        "--hook" => {
            let mut msg2 = String::from(playerinactiveT2.clone());
            let mut msg1 = String::from("");
            let t2 = thread::spawn(move || loop {
                let length = msg2.chars().count();
                if length >= config.hook_mode.scroll_at && !config.hook_mode.scroll_text  {
                    for i in 0..length {
                        if let Ok((msg1_new, msg2_new)) = scrolltext.try_recv() {
                                msg2 = msg2_new.clone();
                                msg1 = msg1_new;
                                msg2.push(' ');
                                break;
    
                        }
                        let result: String = msg2
                            .chars()
                            .cycle()
                            .skip(i)
                            .take(config.hook_mode.scroll_at)
                            .collect();
                        thread::sleep(time::Duration::from_millis(
                            (config.hook_mode.scroll_speed / result.chars().count() as u64)
                                * result.chars().count() as u64,
                        ));
                        println!("{}{}", msg1, result)
                    }
                }else {
                    if let Ok((msg1_new, msg2_new)) = scrolltext.try_recv() {
                        msg2 = msg2_new.clone();
                        msg1 = msg1_new;
                        msg2.push(' ');
                    } else {
                        println!("{}{}", msg1, msg2);
                        thread::sleep(dont_fry_my_cpu)
                    }

                }
            });

            t2.join().expect("Could not start required thread");
            t1.join().expect("Could not start required thread");
        }
        "--hook-waybar" => {
            let mut msg2 = String::from(playerinactiveT2.clone());
            let mut msg1 = String::from("");
            let t2 = thread::spawn(move || loop {
                let length = msg2.chars().count();
                if length >= config.hook_mode.scroll_at && !config.hook_mode.scroll_text {
                    for i in 0..length {
                        if let Ok((msg1_new, msg2_new)) = scrolltext.try_recv() {
                                msg2 = msg2_new.clone();
                                msg1 = msg1_new;
                                msg2.push(' ');
                                break;
    
                        }
                        let result: String = msg2
                            .chars()
                            .cycle()
                            .skip(i)
                            .take(config.hook_mode.scroll_at)
                            .collect();
                        thread::sleep(time::Duration::from_millis(
                            (config.hook_mode.scroll_speed / result.chars().count() as u64)
                                * result.chars().count() as u64,
                        ));
                        println!("{{ \"text\":\"{}{}\" }}", msg1, result);
                    }
                }else {
                    if let Ok((msg1_new, msg2_new)) = scrolltext.try_recv() {
                        msg2 = msg2_new.clone();
                        msg1 = msg1_new;
                        msg2.push(' ');
                    
                    }else {
                        println!("{{ \"text\":\"{}{}\" }}", msg1, msg2);
                        thread::sleep(dont_fry_my_cpu)
                    }
                }
            });

            t2.join().expect("Could not start required thread");
            t1.join().expect("Could not start required thread");
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
            let state_icon = match unwrapedData.status {
                PlaybackStatus::Paused => "󰐊 ",
                PlaybackStatus::Playing => "󰏤 ",
                _ => "",
            };
            println!(
                "{}{} - {}",
                state_icon, unwrapedData.artist, unwrapedData.title
            );
        }
        "--play" => playerctl::play(),
        "--pause" => playerctl::pause(),
        "--next" => playerctl::next(),
        "--previous" => playerctl::previous(),
        "--toggle" => playerctl::toggle(),
        _ => {
            print!("Unregonised Option.")
        }
    }
}
