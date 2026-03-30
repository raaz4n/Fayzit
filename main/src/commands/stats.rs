use serde::{Deserialize, Serialize};
use reqwest;
use std::time::Duration;
use serenity::{all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponseMessage, InteractionResponseFlags, MessageFlags, ResolvedValue}, builder::{CreateEmbed, CreateMessage}};
use crate::config::FACEITTOKEN;
use crate::commands::search::search_user;
use crate::commands::search::getsteamname;

const P_URL: &str = "https://open.faceit.com/data/v4/players";

// struct setup for user information
#[derive(Serialize, Deserialize, Debug)]
pub struct FaceitData {
    player_id: String,
    avatar: String,
    games: Games,
    nickname: String,
    country: String,
    steam_id_64: String,
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

// struct for finding leaderboard ranking
#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    payload: i64,
}

// get current statistics using the username and search type (FACEIT username or custom steam ID)
pub async fn get_current_stats(_message: &str, searchtype: &str) -> CreateInteractionResponseMessage {
    let mut faceiturl = String::new();                                                          // FACEIT API URL

    match searchtype {                                                      
        "faceit-username" => {                                                                  // if the user wants to search by FACEIT username
            let formatteduser = search_user(_message).await;                                    // use the search_user function to search for the proper user
            if formatteduser.is_empty() {                                                       // if the user was not found, send an error message
                return CreateInteractionResponseMessage::new()
                    .content(format!("Sorry, unable to find user \"{_message} \", make sure you entered a proper FACEIT username"))
                    .flags(InteractionResponseFlags::EPHEMERAL);
            }
            faceiturl = format!("{}?game=cs2&nickname={}", P_URL, formatteduser);               // if the user was found, format the URL
        },
        "steam-id" => {                                                                         // if the user wants to search by steam ID
            let steamid = getsteamname(_message).await;                                         // use the getsteamname function to search via steam XML
            if steamid.is_empty() {                                                             // if no user was found, send an error message
                return CreateInteractionResponseMessage::new()
                    .content(format!("Sorry, unable to find user \" {_message} \", make sure you entered a proper Steam ID (either custom URL or ID64"))
                    .flags(InteractionResponseFlags::EPHEMERAL);
            }
            faceiturl = format!("{}?game=cs2&game_player_id={}", P_URL, steamid);               // if the user was found, format the URL
        },
        _ => {}                                                                                 // blank case
    }

    let client = reqwest::Client::builder()                                                     // create a client with a timeout req for 5 secs
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let req = match client.get(&faceiturl)                                                      // GET request on the faceiturl
        .bearer_auth(FACEITTOKEN.clone())                                                       // requires FACEIT bearer auth token
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return CreateInteractionResponseMessage::new()
                .content("Sorry, there was an error trying to get stats (Possibly an API key issue?).");
        }
    };

    let body = match req.text().await {                                                         // get the body from the request text
        Ok(response) => response,
        Err(_) => return CreateInteractionResponseMessage::new(),
    };

    let data: FaceitData = serde_json::from_str(&body).unwrap();                                // use data as the JSON parse from request

    let avatar = data.avatar;                                                                   // set all variables for statistics embed later
    let username = data.nickname;
    let elo = data.games.cs2.faceit_elo.to_string();
    let faceitlvl = data.games.cs2.skill_level.to_string();
    let region = data.games.cs2.region;
    let upperregion = region.to_uppercase();
    let usercountry = data.country;
    let uppercountry = usercountry.to_uppercase();
    let steam_id_64 = data.steam_id_64.to_string();

    let player_id = data.player_id;                                                             // using the player id
    let mut challurl = String::new();                                                           // find the users rank from FACEITs api ranking
    challurl = format!("https://www.faceit.com/api/ranking/v1/globalranking/cs2/{}/{}", upperregion, player_id);
    
    let client2 = reqwest::Client::builder()                                                    // using another client and req
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let req2 = match client2.get(&challurl)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return CreateInteractionResponseMessage::new()
                .content("Sorry, there was an error trying to get stats (Possibly an API key issue?).");
        }
    };

    let body2 = match req2.text().await {
        Ok(response) => response,
        Err(_) => return CreateInteractionResponseMessage::new(),
    };

    let lb: Object = serde_json::from_str(&body2).unwrap();                                     // lb is set to the body to the GET response of the ranking URL
    let rankstr = lb.payload.to_string();
    let rank = rankstr.parse::<i64>().unwrap();                                                 // make sure that the rank from the payload is set to an i64

    let mut regionstring = String::new();
    let mut lvlstring = String::new();

    match faceitlvl.as_str() {                                                                  // based on a users level, change the emote of the level
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

    if rank <= 1000 {
        lvlstring = "<:challenger:1488174686088204419>".to_string();                            // in the special case that a user is rank 1000 or higher, use the challenger emote
    }

    match region.as_str() {                                                                     // change the region emote based on which region the account is set to
        "EU" => {
            regionstring = ":flag_eu:".to_string();
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

    let formatted_username = username.replace("_", "\\_");                                      // in case a username includes multiple underscores, make sure it formats properly to bypass Discord's message formatting
    let mut embeds = CreateEmbed::new()                                                         // create the embed message
        .thumbnail(avatar)                                                                      // avatar thumbnail
        .title(format!("{formatted_username}'s Stats"))                                         // title with the formatted username
        .description(format!("[FACEIT](https://www.faceit.com/en/players/{})\n[Steam](https://steamcommunity.com/profiles/{})", username, steam_id_64)) // links for FACEIT and Steam
        .field("Elo", &elo, true)                                                               // get the elo
        .field("Level", format!("   {}", &lvlstring), true)                                     // level emote
        .field("Region", format!("{} {}", upperregion, regionstring), true)                     // region with the region emote
        .field("Country", format!("{} :flag_{}:", uppercountry, usercountry), true);            // country with the country emote

    if rank <= 1000 {
        embeds = embeds.field("Rank", format!("#{}", rank), true);                              // if the user is within challenger ranking, add another embed field with their rank
    }

    return CreateInteractionResponseMessage::new().embed(embeds);                               // return the embed
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> CreateInteractionResponseMessage {
    let mut username = String::new();                                                      
    let mut search_type = String::new();

    for o in command.data.options() {
        match o.name {
            "faceit-username" => {                                                              // if the user wants to use a FACEIT username to search
                search_type = "faceit-username".to_string();                                    // set search_type to it
                if let ResolvedValue::String(username_str) = o.value {                          // set the username to the value typed by user
                    username = username_str.to_string();
                }
            },
            "steam-id" => {                                                                     // if the user wants to use a Steam ID to search
                search_type = "steam-id".to_string();                                           // set search_type to it
                if let ResolvedValue::String(username_str) = o.value {                          // set the steam ID to the value type by user
                    username = username_str.to_string();
                }
            },
            _ => {}
        }
    }
    
    get_current_stats(&username, &search_type).await
}

pub fn statistics() -> CreateCommand {
    CreateCommand::new("stats")                                                                 // command setup for 'stats'
        .description("Get a users FACEIT stats")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "faceit-username", "Search for the user by FACEIT username").required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::String, "steam-id", "Search for the user by Steam ID").required(false))
}