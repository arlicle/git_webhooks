use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::web::{Bytes, Query};
use serde_json::{json, Map, Value};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use hmac::{Hmac, Mac, NewMac};
use hex;

use crate::config;

type HmacSha1 = Hmac<Sha1>;


#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    cwd: Option<String>,
    command: Option<String>,
    branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VecString(Vec<String>);


pub fn validate(secret: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let mut hmac = HmacSha1::new_varkey(secret).unwrap();
    hmac.update(message);
    hmac.verify(signature).is_ok()
}





pub async fn webhooks_handle(req: HttpRequest, request_body: web::Json<Value>, query_info: web::Query<Info>, config_data: web::Data<Mutex<config::Config>>) -> HttpResponse {
    println!("REQ: {:?}", req);
    println!("REQ: {:?}", req.headers());
    println!("REQ: {:?}", request_body);
    println!("query {:?}", query_info);

    let config_data = config_data.lock().unwrap();


    let mut signature = "";
    if let Some(v) = req.headers().get("x-hub-signature") {
        if let Ok(x) = v.to_str() {
            signature = x.trim_start_matches("sha1=");
        }
    }

    let repository_name:&str = match request_body.pointer("/repository/name") {
        Some(Value::String(v)) => v,
        Some(_) | None => {
            return HttpResponse::Ok().body("Cant not get repository name");
        }
    };
    println!("repository_name {}", repository_name);

    let branch_url:&str = match request_body.pointer("/ref") {
        Some(Value::String(v)) => v,
        Some(_) | None => {
            return HttpResponse::Ok().body("Cant not get repository name");
        }
    };
    let branch_name = branch_url.replace("refs/heads/", "");
    println!("branch_name {}", branch_name);
    if signature != "" {
        println!("signature {}", signature);

        let signature_bytes = match hex::decode(&signature) {
            Ok(result) => result,
            Err(error) => {
                return HttpResponse::Ok().body("Failed");
            }
        };

        let secret = &config_data.get_project_config_data(repository_name, "secret")[0];
        let secret = secret.as_bytes();

        let r = validate(secret, &signature_bytes, request_body.to_string().as_bytes());
        println!("result {}", r);
        if !r {
            return HttpResponse::Ok().body("Signature valid failed");
        }
    }

    let commands = config_data.get_project_config_data(repository_name, "command");
    for command in commands {

    }

    println!("hello");
    HttpResponse::Ok().body("Done")
}