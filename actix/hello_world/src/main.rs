use actix_web::{
    get, post,
    web::{self, route},
    App, HttpResponse, HttpServer, Responder,
};

struct Hello {
    first: String,
    second: String,
}

impl Hello {
    fn new(first: String, second: String) -> Self {
        Self { first, second }
    }
    async fn call_hello(&self) -> impl Responder {
        HttpResponse::Ok().body(format!("{} {}", self.first, self.second))
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hallo World !")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    Hello::new("hallo".to_string(), "world".to_string())
        .call_hello()
        .await
    // HttpResponse::Ok().body("Manual Hello World !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route(
                "another_hello",
                web::get().to(|| async {
                    Hello::new("hallo".to_string(), "world".to_string())
                        .call_hello()
                        .await
                }),
            )
            .route(
                "/hello",
                web::get().to(move || {
                    let h: Hello = Hello {
                        first: String::from("hallo"),
                        second: String::from("world"),
                    };

                    async move { h.call_hello().await }
                }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
