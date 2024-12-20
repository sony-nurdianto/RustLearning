use actix_web::{web, App, HttpServer};
use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::routes::messages::messages_routes;

struct Database {
    db: Surreal<Client>,
}

impl Database {
    async fn init() -> Self {
        let db: Surreal<Client> = Surreal::new::<Ws>("127.0.0.1:8080").await.unwrap();

        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await.unwrap();
        Self { db }
    }
}

// async fn sfunction(data: Data<Database>) -> impl Responder {
//     let select_users = data.db.select("users");
// }

static DB: Lazy<Surreal<Client>> = Lazy::new(|| Surreal::init());

#[tokio::main]
pub async fn tokio_main() -> Result<(), std::io::Error> {
    let newdb: Database = Database::init().await;

    DB.connect::<Ws>("someing").await.unwrap();
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();

    // Select a specific namespace / database
    DB.use_ns("test").use_db("test").await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(DB.clone()))
            .configure(messages_routes)
    })
    .bind(("0.0.0.0", 8080));

    match server {
        Ok(bind_server) => {
            println!("{}", format!("Server Runing At {}", bind_server.addrs()[0]));
            bind_server.run().await
        }
        Err(err) => return Err(err),
    }
}
