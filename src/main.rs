use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse};

use serde_json::{json, Map, Value};
use hex_literal::hex;
use sha1::{Sha1, Digest};



#[post("/git_post_receive")]
async fn git_post_receive(req: HttpRequest, request_body: web::Json<Value>) -> HttpResponse {
    println!("REQ: {:?}", req);
    println!("REQ: {:?}", req.headers());
    println!("request_body:\n{:?}", request_body);

//    let mut hasher = Sha1::new();
//    hasher.update(request_body.as_str().unwrap().as_bytes());
    let s = Sha1::digest(request_body.as_str().unwrap().as_bytes());
    println!("{:?}", s);

    HttpResponse::Ok().body("done")
}

#[get("/")]
async fn index() -> HttpResponse {

    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(git_post_receive)
        .service(index))
        .bind("0.0.0.0:9005")?
        .run()
        .await
}


