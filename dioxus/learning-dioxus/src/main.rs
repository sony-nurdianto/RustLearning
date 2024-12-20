#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod models;
use components::story_component::StoryListing;
use models::story_model::StoryItem;

pub fn App() -> Element {
    rsx! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Author".to_string(),
                score: 0,
                descendants: 0,
                time: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
        }
    }
}

fn main() {
    launch(App);
}
