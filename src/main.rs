use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse};

use serde_json::{json, Map, Value};




#[post("/git_post_receive")]
async fn git_post_receive(req: HttpRequest, request_body: web::Json<Value>) -> HttpResponse {
    println!("REQ: {:?}", req);
    println!("request_body:\n{:?}", request_body);

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


