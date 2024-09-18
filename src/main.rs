use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn testing_get() -> impl Responder {
    HttpResponse::Ok().body("Hello world 2!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(testing_get)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}