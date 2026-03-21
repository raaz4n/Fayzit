use serde::{Deserialize, Serialize};
use reqwest;
use std::time::Duration;
use crate::config::FACEITTOKEN;

const S_URL: &str = "https://open.faceit.com/data/v4/search/players?";

// deserialization of the JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchData {
    items: Vec<SearchItems>, // going through a vector of nicknames once search is called to find the right user
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchItems {
    nickname: String,
}

pub async fn search_user(_message: &str) -> String {                    // searches user based on FACEIT username
    let name: String = _message.to_string();
    
    let search_url: String = format!("{}nickname={}", S_URL, name);     // creates a new URL string based on user input of nickname

    let client: reqwest::Client = builder()                             // client timeout
        .timeout(Duration::from_secs(5))
        .build
        .unwrap();
    let req: String = reqwest::get(&search_url)                         // get request to that new URL
        .await?
        .text()
        .await?;

    let bearer: String = format!("Bearer {}", *FACEITTOKEN);
    

}
