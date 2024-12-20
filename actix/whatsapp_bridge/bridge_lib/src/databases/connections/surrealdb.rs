use core::panic;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub type MainDB = Surreal<Client>;

pub struct SurrealDatabase {
    pub database: MainDB,
}

impl SurrealDatabase {
    pub async fn connect() -> Self {
        let db_connect: Result<Surreal<Client>, surrealdb::Error> = Surreal::new::<Ws>("").await;
        match db_connect {
            Ok(db) => {
                let signin = db
                    .signin(Root {
                        username: "root",
                        password: "password",
                    })
                    .await;
                if let Err(signin_err) = signin {
                    panic!("Failed To SignIn {signin_err}")
                }
                let environtment = db.use_ns("test").use_db("test").await;
                if let Err(env) = environtment {
                    panic!("Failed To Select Environtment {env}");
                }

                Self { database: db }
            }
            Err(err) => panic!("Failed To Connected To DB {err}"),
        }
    }
}
