#[macro_use] extern crate rocket;

mod deck;
use deck::*;

use std::{
    path::Path, sync::Arc
};
use rocket::{fs::NamedFile, State};

#[get("/deck_metadata/<name>")]
async fn deck_metadata(decks: &State<Arc<Vec<Deck>>>, name: &str) -> Option<String> {
    serde_json::to_string(
        decks.iter()
            .find(|deck| deck.name == name)?
        ).ok()
}

#[get("/deck_names")]
fn deck_names(decks: &State<Arc<Vec<Deck>>>) -> String {
    decks.iter()
        .map(|deck| deck.name.clone() + "\n")
        .collect()
}

#[get("/visual/<name>")]
async fn get_visual(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Visuals/{name}"))).await.ok()
}

#[get("/sound/<name>")]
async fn get_sound(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Sounds/{name}"))).await.ok()
}



#[launch]
fn rocket() -> _ {
    let decks = std::fs::read_dir("decks/Decks").unwrap()
        .map(|path| {
            let reader = std::fs::File::open(path.unwrap().path()).unwrap();
            serde_json::from_reader(reader).unwrap()
        })
        .collect::<Vec<Deck>>();
    rocket::build().mount("/", routes![
        deck_metadata,
        deck_names,
        get_visual,
        get_sound,
    ]).manage(Arc::new(decks))
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    use super::Deck;

    #[test]
    fn get_decks() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::deck_names)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        for deck_name in response.into_string().unwrap().lines() {
            let response = client.get(uri!(super::deck_metadata(deck_name))).dispatch();
            assert_eq!(response.status(), Status::Ok);
        }
    }

    #[test]
    fn visual_files_integrity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let decks = std::fs::read_dir("decks/Decks").unwrap()
            .map(|path| {
                let reader = std::fs::File::open(path.unwrap().path()).unwrap();
                serde_json::from_reader(reader).unwrap()
            })
            .collect::<Vec<Deck>>();

        for deck in decks {
            for card in deck.cards {
                let response = client.get(uri!(super::get_visual(card.visual))).dispatch();
                assert_eq!(response.status(), Status::Ok);
            }
        }
    }

    #[test]
    fn audio_files_integrity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let decks = std::fs::read_dir("decks/Decks").unwrap()
            .map(|path| {
                let reader = std::fs::File::open(path.unwrap().path()).unwrap();
                serde_json::from_reader(reader).unwrap()
            })
            .collect::<Vec<Deck>>();

        for deck in decks {
            for card in deck.cards {
                let response = client.get(uri!(super::get_sound(card.audio))).dispatch();
                assert_eq!(response.status(), Status::Ok);
            }
        }
    }
}
