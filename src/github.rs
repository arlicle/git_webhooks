use std::sync::Mutex;
use std::process::Command;
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





pub async fn webhooks_handle(req: HttpRequest, request_body_bytes: Bytes, query_info: web::Query<Info>, config_data: web::Data<Mutex<config::Config>>) -> HttpResponse {
    let request_body = std::str::from_utf8(&request_body_bytes[..]).unwrap();
    let request_body:Value = serde_json::from_str(request_body).unwrap();
    let config_data = config_data.lock().unwrap();


    println!("REQ: {:?}", req);
    println!("REQ: {:?}", req.headers());
    println!("REQ: {:?}", request_body);
    println!("query {:?}", query_info);


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

    let branch_url:&str = match request_body.pointer("/ref") {
        Some(Value::String(v)) => v,
        Some(_) | None => {
            return HttpResponse::Ok().body("Cant not get repository name");
        }
    };
    let branch_name = branch_url.replace("refs/heads/", "");
    if signature != "" {

        let signature_bytes = match hex::decode(&signature) {
            Ok(result) => result,
            Err(error) => {
                return HttpResponse::Ok().body("Failed");
            }
        };

        let secret = &config_data.get_project_config_data(repository_name, "secret")[0];
        let secret = secret.as_bytes();

        let r = validate(secret, &signature_bytes, &request_body_bytes[..]);
        if !r {
            return HttpResponse::Ok().body("Signature valid failed");
        }
    }

    let mut commands = config_data.get_project_config_data(repository_name, "command");
    if commands.len() == 1 && &commands[0] == "" {
        match &query_info.command {
            Some(v) => {
                commands = vec![v.replace("+", " ")];
            },
            None => {
                commands = vec![];
            }
        }
    }

    for command in &commands {
        let s: Vec<&str> = command.split(" ").collect();
        let mut echo_hello = Command::new(s[0]);
        if s.len() > 1 {
            echo_hello.args(&s[1..]);
        }
        let aaa = echo_hello.output().expect("failed to execute process");
        let request_body = std::str::from_utf8(&aaa.stdout).unwrap();
        println!("{}", request_body);

    }

    HttpResponse::Ok().body("Done")
}