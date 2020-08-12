use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse,FromRequest};
use actix_web::web::{Bytes, Query};
use serde_json::{json, Map, Value};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use hmac::{Hmac, Mac, NewMac};
use hex;
use std::sync::Mutex;
mod github;
mod config;
mod executor;
use std::sync::mpsc;
use crossbeam_channel::{unbounded,Sender,Receiver};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_data = web::Data::new(Mutex::new(config::Config::new()));
    let task_data = web::Data::new(executor::Task::run());
    HttpServer::new(move|| App::new()
        .app_data(config_data.clone())
        .app_data(task_data.clone())
        .service(web::resource("/webhooks/git").route(web::post().to(github::webhooks_handle)))
        .service(index))
        .bind("0.0.0.0:9005")?
        .run()
        .await
}


#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("hello")
}


