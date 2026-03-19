use serde::{Deserialize, Serialize};


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

pub async fn search_user(_message: &str) -> String {
    let test = String::new();
    return test;
}