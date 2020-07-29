use actix_web::{post, web, App, HttpServer, HttpResponse};

use serde_json::{json, Map, Value};




#[post("/git_post_receive")]
async fn git_post_receive(request_body: web::Json<Value>) -> HttpResponse {
    println!("request_body:\n{:?}", request_body);

    HttpResponse::Ok().body("done")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(git_post_receive))
        .bind("0.0.0.0:9005")?
        .run()
        .await
}


