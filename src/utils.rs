use mpris::{PlayerFinder, Player, PlaybackStatus};
pub struct MetaData {
    pub status: String,
    pub title: String,
    pub artist: String,
    pub playing: bool,
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