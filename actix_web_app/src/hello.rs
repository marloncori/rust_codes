use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\t Listening on port 8080...");
    HttpServer::new(|| {
        App::new().route("/hello", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
