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
use serde::Deserialize;
use serde_ini::from_str;
use std::fs;

fn bool_from_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "yes" | "true" => Ok(true),
        "no" | "false" => Ok(false),
        _ => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&s),
            &"either \"yes\" or \"no\"",
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    #[serde(rename = "hook-mode")]
    pub hook_mode: HookMode,
    #[serde(rename = "discord-rpc")]
    pub discord_rpc: DiscordRPC,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub dont_fry_my_cpu: u64,
}

#[derive(Debug, Deserialize)]
pub struct HookMode {
    #[serde(rename = "play-icon")]
    pub play_icon: String,
    #[serde(rename = "paused-icon")]
    pub paused_icon: String,
    #[serde(rename = "stopped-icon")]
    pub stopped_icon: String,
    pub format: String,
    #[serde(rename = "format-inactive")]
    pub format_inactive: String,
    #[serde(rename = "scroll-text", deserialize_with = "bool_from_yes_no")]
    pub scroll_text: bool,
    #[serde(rename = "scroll-at")]
    pub scroll_at: usize,
    #[serde(rename = "scroll-speed")]
    pub scroll_speed: u64,
}

#[derive(Debug, Deserialize)]
pub struct DiscordRPC {
    #[serde(deserialize_with = "bool_from_yes_no")]
    pub enable: bool,
}

pub fn defaultConfig() -> Config {
    return Config {
        general: General {
            dont_fry_my_cpu: 500,
        },
        hook_mode: HookMode {
            play_icon: "󰐊 ".to_owned(),
            paused_icon: "󰏤 ".to_owned(),
            stopped_icon: "󰤄 ".to_owned(),
            format: "[artist] - [title]".to_owned(),
            format_inactive: "Player not Active".to_owned(),
            scroll_text: true,
            scroll_at: 30,
            scroll_speed: 150,
        },
        discord_rpc: DiscordRPC { enable: false },
    };
}

pub fn readConfig(configPath: &str) -> Config {
    let contents = fs::read_to_string(configPath).expect("Error: Failed to read Config file");
    let config: Config = from_str(&contents).expect("Error: Failed to parse INI Config file");
    return config;
}
