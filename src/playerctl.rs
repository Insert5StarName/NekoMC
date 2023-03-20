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
use mpris::{PlaybackStatus, Player};

use crate::utils;

fn failWithOutPlayer() -> Player {
    return utils::get_player().expect("Could not get Player.");
}

pub fn play() {
    failWithOutPlayer()
        .play()
        .expect("Could not get Player to resume.");
    println!("Player is resuming playback.");
}

pub fn pause() {
    failWithOutPlayer()
        .pause()
        .expect("Could not get Player to pause.");
    println!("Player is pausing playback.");
}

pub fn next() {
    failWithOutPlayer()
        .next()
        .expect("Could not get Player to skip the current song.");
    println!("Player is skipping the current song.");
}
pub fn previous() {
    failWithOutPlayer()
        .previous()
        .expect("Could not get Player to return to previous song.");
    println!("Player is returning to previous song.");
}
pub fn toggle() {
    let status = utils::get_metadata(failWithOutPlayer())
        .expect("Could not get State of Player.")
        .status;
    match status {
        PlaybackStatus::Paused => {
            failWithOutPlayer()
                .play()
                .expect("Could not get the Player to resume playing.");
            println!("Player Resume Sucess.");
        }
        PlaybackStatus::Playing => {
            failWithOutPlayer()
                .pause()
                .expect("Could not get Player to pause.");
            println!("Player is pausing playback.");
        }
        PlaybackStatus::Stopped => println!("Can't resume stopped Player"),
    };
}
