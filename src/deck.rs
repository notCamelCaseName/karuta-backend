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
