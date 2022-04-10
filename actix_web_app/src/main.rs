/* Practical Rust Web Programming, continue from page 26/27*/
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
/*
use actix_files::NamedFile;
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}*/

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Catdex",
        "cats": [
            {
                "name": "British short hair",
                "image_path": "/static/image/british-short-hair.jpg"
            },
            {
                "name": "Persian",
                "image_path": "/static/image/persian.jpg"
            },
            {
                "name": "Ragdoll",
                "image_path": "/static/image/ragdoll.jpg"
            },
            {
                "name": "Japenese",
                "image_path": "/static/image/japanese.jpg"
            },
            {
                "name": "Generic black",
                "image_path": "/static/image/black.jpg"
            },
            {
                "name": "Siamese",
                "image_path": "/static/image/siames.jpg"
            }
        ]
    });

    let contents = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = String::from("127.0.0.1:8080");
    //Handlebars is needed to handle the index.html
    //which has been conveted into a template to avoid repetition
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./static")
    .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("\t Listening on port 8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new(
                "/static", "static")
                    .show_files_listing(),
            )
            .route("/", 
                web::get().to(index))
    })
    .bind(url)?
    .run()
    .await
}