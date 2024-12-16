use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Deck {
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub cover: String,
    pub cards: Vec<Card>,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub anime: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub visual: String,
    pub audio: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub name: String,
    pub icon: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CategoryJSON {
    pub categories: Vec<Category>,
    pub types: Vec<String>,
}
