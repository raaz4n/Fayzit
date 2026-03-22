use serde::{Deserialize, Serialize};
use reqwest;
use std::time::Duration;
use crate::config::FACEITTOKEN;

const S_URL: &str = "https://open.faceit.com/data/v4/search/players?";  // faceit API url

// deserialization of the JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchData {
    items: Vec<SearchItems>,                                            // going through a vector of nicknames once search is called to find the right user
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchItems {
    nickname: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "profile")]
pub struct SteamData {
    #[serde(rename = "steamID64")]
    steamid64: String,
}

pub async fn search_user(_message: &str) -> String {                    // searches user based on FACEIT username
    let name: String = _message.to_string();
    
    let search_url: String = format!("{}nickname={}", S_URL, name);     // creates a new URL string based on user input of nickname
    
    let client = reqwest::Client::builder()                             // client timeout
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let req = match client.get(&search_url)                             // get request to that new URL
        .bearer_auth(FACEITTOKEN.clone())                               // bearer auth for access to FACEIT API
        .send()
        .await 
    {
        Ok(response) => response,
        Err(error) => {
            println!("Bad request: {error}");
            return String::new();
        }
    };

    let body = match req.text().await {
        Ok(response) => response,
        Err(_) => return String::new(),
    };

    let data: SearchData = serde_json::from_str(&body).unwrap();       // get the nicknames from the JSON parse

    let mut temp = String::new();
    let count = data.items.len();                                       // if there are no users returned by the search
    if count == 0 {                                                     // make temp return an empty string
        {}
    }
    else {                                                              // else, preemptively set it to the first user in the search
        temp = data.items[0].nickname.to_string();
    }

    let mut searchrange = count;                                        // set the searchrange var to len of users returned by FACEIT search
    if count >= 20 {                                                    // if it's greater than 20, just search the first 20 users.
        searchrange = 20;
    }

    'out:                                                               // pull info from list of users based on search
        for i in 0..searchrange {
            let playername = data.items[i].nickname.to_string();
            if playername.to_lowercase() == _message.to_lowercase() {   // if the exact username is found, assign it to temp and break out
                temp = playername;
                break 'out;
            }
        }

    return temp;                                                        // return username from search or empty string based on if user exists or not
}

pub async fn getsteamname(_message: &str) -> String {                   // gets steam ID64
    let mut name = String::new();

    'link:
        for i in 0..2 {
            let steamurl =
                if i == 0 {                                                 // use steams API with XML based on user input
                    format!("http://steamcommunity.com/id/{}/?xml=1", _message)
                }
                else {
                    format!("http://steamcommunity.com/profiles/{}/?xml=1", _message)
                };

            let client = reqwest::Client::builder()                     // client timeout
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap();
            
            let req = match client.get(&steamurl)
                .send()
                .await
            {
                Ok(response) => response,
                Err(error) => {
                    println!("Bad request: {error}");
                    return String::new();
                }
            };

            let body = match req.text().await {
                Ok(response) => response,
                Err(_) => return String::new(),
            };

            let data: SteamData = quick_xml::de::from_str(&body).unwrap();
            name = data.steamid64;                                      // get the ID64 from the user to search FACEITs API
            if name.is_empty(){
                break 'link;
            }
        }
    
    return name;
}