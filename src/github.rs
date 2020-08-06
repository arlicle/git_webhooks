use std::process::Command;
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


#[derive(Serialize,Deserialize,Debug)]
pub struct Info {
    cwd: Option<String>,
    command: Option<String>,
    branch: Option<String>,
}




pub fn validate(secret: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let mut hmac = HmacSha1::new_varkey(secret).unwrap();
    hmac.update(message);
    hmac.verify(signature).is_ok()
}



pub async fn webhooks_handle(req: HttpRequest, request_body: Bytes, query_info: web::Query<Info>, config_data: web::Data<Mutex<config::Config>>,) -> HttpResponse {
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

    if signature == "" {
        return HttpResponse::Ok().body("Failed");
    }

    println!("signature {}", signature);

    let signature_bytes = match hex::decode(&signature) {
        Ok(result) => result,
        Err(error) => {
            return HttpResponse::Ok().body("Failed");
        }
    };


    let secret = match config_data.secret {
        Some(ref s) => s.as_bytes(),
        None => "".as_bytes()
    };

    let r = validate(secret, &signature_bytes, &request_body[..]);
    println!("result {}", r);
    HttpResponse::Ok().body("Done")
}