#[macro_use] extern crate rocket;

use std::{
    path::Path,
    fs::read_dir,
};
use rocket::fs::NamedFile;

#[get("/deck_metadata/<name>")]
async fn deck_metadata(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Decks/Deck {name}.txt"))).await.ok()
}

#[get("/deck_names")]
async fn deck_names() -> String {
    read_dir("decks/Casual Karuta").unwrap()
        .chain(read_dir("decks/Decks").unwrap())
        .map(|direntry| direntry.unwrap().file_name().into_string().unwrap()) 
        .filter(|filename| filename.starts_with("Deck "))
        .map(|filename| filename[5..].strip_suffix(".txt").unwrap().to_owned() + "\n")
        .collect()
}

#[get("/visual/<name>")]
async fn get_visual(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Visuals/{name}.png"))).await.ok()
}

#[get("/sound/<name>")]
async fn get_sound(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Sound/{name}.mp3"))).await.ok()
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/get", routes![
        deck_metadata,
        deck_names,
        get_visual,
        get_sound,
    ])
}
