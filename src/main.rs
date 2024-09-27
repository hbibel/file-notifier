mod adapters;
mod application;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/live")]
async fn live() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(live))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
