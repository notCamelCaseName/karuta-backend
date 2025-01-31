#[macro_use]
extern crate rocket;

mod deck;
use deck::*;

mod cors;
use cors::*;

use rocket::{fs::NamedFile, State};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::{make_swagger_ui, SwaggerUIConfig}};
use std::{path::Path, sync::Arc};

#[openapi(tag = "Decks")]
#[get("/deck/metadata/<name>")]
async fn deck_metadata(decks: &State<Arc<Vec<Deck>>>, name: &str) -> Option<String> {
    serde_json::to_string(decks.iter().find(|deck| deck.name == name)?).ok()
}

#[openapi(tag = "Decks")]
#[get("/deck/names")]
fn deck_names(decks: &State<Arc<Vec<Deck>>>) -> String {
    decks.iter().map(|deck| deck.name.clone() + "\n").collect()
}

#[openapi(tag = "Decks")]
#[get("/visual/<name>")]
async fn get_visual(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Visuals/{name}")))
        .await
        .ok()
}

#[openapi(tag = "Decks")]
#[get("/sound/<name>")]
async fn get_sound(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Sounds/{name}")))
        .await
        .ok()
}

#[openapi(tag = "Decks")]
#[get("/deck/cover/<name>")]
async fn get_cover(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Covers/{name}")))
        .await
        .ok()
}

#[openapi(tag = "Themes")]
#[get("/theme/names")]
fn theme_names() -> String {
    std::fs::read_dir("decks/Themes")
        .unwrap()
        .map(|rd| rd.unwrap().file_name().into_string().unwrap())
        .filter(|filename| filename.contains(".json"))
        .map(|s| s + "\n")
        .collect()
}

#[openapi(tag = "Themes")]
#[get("/theme/<name>")]
async fn get_theme(name: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("decks/Themes/{name}")))
        .await
        .ok()
}

#[openapi(tag = "Categories")]
#[get("/categories")]
async fn get_categories(categories: &State<Arc<CategoryJSON>>) -> Option<String> {
    serde_json::to_string(&categories.categories).ok()
}

#[openapi(tag = "Categories")]
#[get("/types")]
async fn get_types(categories: &State<Arc<CategoryJSON>>) -> Option<String> {
    serde_json::to_string(&categories.types).ok()
}

#[openapi(tag = "Categories")]
#[get("/categories_and_types")]
async fn get_categories_and_types(categories: &State<Arc<CategoryJSON>>) -> Option<String> {
    serde_json::to_string(categories.inner().as_ref()).ok()
}

#[openapi(tag = "Categories")]
#[get("/category/icon/<name>")]
async fn get_category_icon(name: &str, categories: &State<Arc<CategoryJSON>>) -> Option<NamedFile> {
    let icon_path = &categories
        .categories
        .iter()
        .find(|category| category.name == name)?
        .icon;
    NamedFile::open(Path::new(&format!("decks/Categories/{icon_path}")))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    let decks = std::fs::read_dir("decks/Decks")
        .unwrap()
        .map(|path| {
            let reader = std::fs::File::open(path.unwrap().path()).unwrap();
            serde_json::from_reader(reader).unwrap()
        })
        .collect::<Vec<Deck>>();
    let categories: CategoryJSON =
        serde_json::from_reader(std::fs::File::open("decks/Categories/Categories.json").unwrap())
            .unwrap();

    rocket::build()
        .attach(CORS)
        .mount(
            "/",
            openapi_get_routes![
                deck_metadata,
                deck_names,
                theme_names,
                get_visual,
                get_sound,
                get_cover,
                get_theme,
                get_categories,
                get_types,
                get_categories_and_types,
                get_category_icon,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(Arc::new(decks))
        .manage(Arc::new(categories))
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use super::{CategoryJSON, Deck};

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
        let decks = std::fs::read_dir("decks/Decks")
            .unwrap()
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
        let decks = std::fs::read_dir("decks/Decks")
            .unwrap()
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

    #[test]
    fn category_files_integrity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let categories: CategoryJSON = serde_json::from_reader(
            std::fs::File::open("decks/Categories/Categories.json").unwrap(),
        )
        .unwrap();

        for category in categories.categories {
            let response = client
                .get(uri!(super::get_category_icon(category.name)))
                .dispatch();
            assert_eq!(response.status(), Status::Ok);
        }
    }
}
