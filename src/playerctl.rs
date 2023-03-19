use mpris::Player;

use crate::utils;

fn failWithOutPlayer() -> Player{
    return utils::get_player().expect("Could not get Player.");
}

pub fn play(){ 
    failWithOutPlayer().play().expect("Could not get Player to resume.");
    println!("Player is resuming playback.");
}

pub fn pause(){
    failWithOutPlayer().pause().expect("Could not get Player to pause.");
    println!("Player is pausing playback.");
}

pub fn next(){
    failWithOutPlayer().next().expect("Could not get Player to skip the current song.");
    println!("Player is skipping the current song.");
}
pub fn previous(){
    failWithOutPlayer().previous().expect("Could not get Player to return to previous song.");
    println!("Player is returning to previous song.");
}
pub fn toggle(){
    let isPlaying = utils::get_metadata(failWithOutPlayer()).expect("Could not get State of Player.").playing;
    if isPlaying {
        failWithOutPlayer().pause().expect("Could not pause the Player.");
        println!("Paused Player Success.");
    } else {
        failWithOutPlayer().play().expect("Could not get the Player to resume playing.");
        println!("Player Resume Sucess.");
    }
}