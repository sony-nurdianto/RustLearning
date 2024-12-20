use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Client;
use wither::{prelude::*, Result};

// Define a model. Simple as deriving a few traits.
#[derive(Debug, Model, Deserialize, Serialize)]
pub struct Favorite {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub r#type: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct FavoriteCollection {
    pub data: Vec<Favorite>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Connect & sync indexes.
    let db = Client::with_uri_str("mongodb://localhost:27017/")
        .await?
        .database("swfavorites");
    Favorite::sync(&db).await?;

    // Fetch all users.
    let mut cursor = Favorite::find(&db, None, None).await?;
    while let Some(fav) = cursor.next().await {
        println!("{:?}", fav);
    }

    Ok(())
}
