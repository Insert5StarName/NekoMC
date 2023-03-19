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
use mpris::{PlayerFinder, Player, PlaybackStatus};
pub struct MetaData {
    pub title: String,
    pub artist: String,
    pub status: PlaybackStatus,
}


pub fn get_player() -> Option<Player> {
    let finder = PlayerFinder::new().ok();
    if finder.is_none() {
        return None;
    }
    let player = finder.as_ref().unwrap().find_active().ok();
    if player.is_none() {
        return None
    }
    return Some(player.unwrap());
}

pub fn get_metadata(player : Player) -> Option<MetaData> {
    let metadataRaw = player.get_metadata().ok();
    if metadataRaw.is_none() {
        return None;
    }
    let metadata = metadataRaw.unwrap();
    let title = metadata.title().unwrap_or("Unknown title");
    let artist = metadata.artists().unwrap_or((&["Unknown Artist"]).to_vec());
    let status = player.get_playback_status().ok();
    if status.is_none() {
        return None;
    }
    return Some(MetaData {
        title: title.to_string(),
        artist: artist.join(", "),
        status: status.unwrap(),
    });


}