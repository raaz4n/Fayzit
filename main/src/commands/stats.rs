use serde::{Deserialize, Serialize};
use reqwest;
use std::time::Duration;
use serenity::{all::{CommandOptionType, CreateCommandOption, CreateCommand, MessageFlags}, builder::{CreateEmbed, CreateMessage}};
use crate::config::FACEITTOKEN;
use crate::commands::search::search_user;
use crate::commands::search::getsteamname;

const P_URL: &str = "https://open.faceit.com/data/v4/players";
const L_URL: &str = "https://open.faceit.com/data/v4/leaderboards";

#[derive(Serialize, Deserialize, Debug)]
pub struct FaceitData {
    avatar: String,
    games: Games,
    nickname: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Games {
    cs2: Cs2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cs2 {
    faceit_elo: i64,
    region: String,
    skill_level: i64,
    skill_level_label: String,
}

pub async fn get_current_stats(_message: &str, searchtype: &str) -> CreateMessage {
    let mut faceiturl = String::new();
    // let mut leaderboardurl = String::new();  (this will come later for leaderboard spots)

    match searchtype {
        "faceit-username" => {
            let formatteduser = search_user(_message).await;
            if formatteduser.is_empty() {
                return CreateMessage::new()
                    .content(format!("Sorry, unable to find user \"{_message} \", make sure you entered a proper FACEIT username"))
                    .flags(MessageFlags::EPHEMERAL);
            }
            faceiturl = format!("{}?game=cs2&nickname={}", P_URL, formatteduser);
        },
        "steam-id" => {
            let steamid = getsteamname(_message).await;
            if steamid.is_empty() {
                return CreateMessage::new()
                    .content(format!("Sorry, unable to find user \" {_message} \", make sure you entered a proper Steam ID (either custom URL or ID64"))
                    .flags(MessageFlags::EPHEMERAL);
            }
            faceiturl = format!("{}?game=cs2&game_player_id={}", P_URL, steamid);
        },
        _ => {}
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let req = match client.get(&faceiturl)
        .bearer_auth(FACEITTOKEN.clone())
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return CreateMessage::new()
                .content("Sorry, there was an error trying to get stats (Possibly an API key issue?).");
        }
    };

    let body = match req.text().await {
        Ok(response) => response,
        Err(_) => return CreateMessage::new(),
    };

    let data: FaceitData = serde_json::from_str(&body).unwrap();

    let avatar = data.avatar;
    let username = data.nickname;
    let elo = data.games.cs2.faceit_elo.to_string();
    let faceitlvl = data.games.cs2.skill_level.to_string();
    let region = data.games.cs2.region;
    let upperregion = region.to_uppercase();
    let usercountry = data.country;
    let uppercountry = usercountry.to_uppercase();

    let mut regionstring = String::new();
    let mut lvlstring = String::new();

    match faceitlvl.as_str() {
        "1" => {
            lvlstring = "<:1_:1456394440460468346>".to_string();
        },
        "2" => {
            lvlstring = "<:2_:1456394439303102485>".to_string();
        },
        "3" => {
            lvlstring = "<:3_:1456394438145478799>".to_string();
        },
        "4" => {
            lvlstring = "<:4_:1456394436987715666>".to_string();
        },
        "5" => {
            lvlstring = "<:5_:1456394435972698356>".to_string();
        },
        "6" => {
            lvlstring = "<:6_:1456394434643230773>".to_string();
        },
        "7" => {
            lvlstring = "<:7_:1456394433745653925>".to_string();
        },
        "8" => {
            lvlstring = "<:8_:1456394431912480778>".to_string();
        },
        "9" => {
            lvlstring = "<:9_:1456394430612504576>".to_string();
        },
        "10" => {
            lvlstring = "<:10:1456394429362475283>".to_string();
        },
        _ => {}
    }

    match region.as_str() {
        "EU" => {
            regionstring = "flag_eu:".to_string();
        },
        "NA" => {
            regionstring = "<:NA:1456425380792500504>".to_string();
        },
        "SA" => {
            regionstring = "<:SA:1456425367526052107>".to_string();
        },
        "OCE" => {
            regionstring = "<:OCE:1456425571213905920>".to_string();
        },
        "SEA" => {
            regionstring = "<:SEA:1456425728252842034>".to_string();
        },
        _ => {}
    }

    let embeds = CreateEmbed::new()
        .thumbnail(avatar)
        .title(format!("{username}'s Stats"))
        .description(format!("[FACEIT]https://www.faceit.com/en/players/{username}"))
        .field("Elo", &elo, true)
        .field("Level", format!("   {}", &lvlstring), true)
        .field("Region", format!("{} {}", upperregion, regionstring), true)
        .field("Country", format!("{} :flag_{}:", uppercountry, usercountry), true);

    return CreateMessage::new().embed(embeds);
}

pub fn statistics() -> CreateCommand {
    CreateCommand::new("stats")
        .description("Get a users FACEIT stats")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "faceit-username", "Search for the user by FACEIT username").required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::String, "steam-id", "Search for the user by Steam ID").required(false))
}