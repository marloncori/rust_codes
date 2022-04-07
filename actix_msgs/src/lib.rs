#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpServer, Result};
use serde::{Serialize, Deserialize};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

pub struct MessageApp {
    pub port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let messages = Arc::new(Mutex::new(vec![]));
        println!("\n\tStarting HTTP server: 127.0.0.1:{} ...", self.port);
        HttpServer::new(move || {
            App::new()
               .data(AppState {
                   server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                   request_count: Cell::new(0),
                   messages: messages.clone(),
               })
               .wrap(middleware::Logger::default())
                .service(index)
                .service(
                    web::resource("/send")
                       .data(web::JsonConfig::default().limit(4096))
                        .route(web::post().to(post)),
                )
                .service(clear)
        })
        .bind(("127.0.0.1", self.port))?
         .workers(8)
           .run()
    }
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
        state.request_count.set(request_count);
    let msgs = state.messages.lock().unwrap();
    
       Ok(web::Json(IndexResponse {
           server_id: state.server_id,
           request_count,
           messages: msgs.clone(),
       }))
}

// method that accepts data
#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

// input - output handler
fn post(msg: web::Json<PostInput>, state: web::Data<AppState>)
                         -> Result<web::Json<PostResponse>> {
   let request_count = state.request_count.get() + 1;
   state.request_count.set(request_count);
   let mut msgs = state.messages.lock().unwrap();
    msgs.push(msg.message.clone());

    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        message: msg.message.clone(),
    }))
}

#[post("/clear")]
fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut msg = state.messages.lock().unwrap();
     msg.clear();
    
     Ok(web::Json(IndexResponse {
         server_id: state.server_id,
         request_count,
         messages: vec![],
     }))
}
