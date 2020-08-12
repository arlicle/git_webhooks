use std::sync::Mutex;
use std::process::Command;
use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::web::{Bytes, Query};
use serde_json::{json, Map, Value};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use sha1::Sha1;
use hmac::{Hmac, Mac, NewMac};
use hex;


use crossbeam_channel::{unbounded, Sender, Receiver};


use crate::config;
use crate::executor;

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


pub async fn webhooks_handle(req: HttpRequest, request_body_bytes: Bytes, query_info: web::Query<Info>, config_data: web::Data<Mutex<config::Config>>, task: web::Data<executor::Task>) -> HttpResponse {
    let request_body = std::str::from_utf8(&request_body_bytes[..]).unwrap();
    let request_body: Value = serde_json::from_str(request_body).unwrap();
    let config_data = config_data.lock().unwrap();

    // 获取github signature
    let mut signature = "";
    if let Some(v) = req.headers().get("x-hub-signature") {
        if let Ok(x) = v.to_str() {
            signature = x.trim_start_matches("sha1=");
        }
    }

    println!("request_body {:?}", request_body);
    println!("request_body {:?}", request_body.pointer("/repository/name"));

    // 获取repository name
    let repository_name:&str = match request_body.pointer("/repository/name") {
        Some(Value::String(v)) => v,
        Some(_) | None => {
            return HttpResponse::Ok().body("Cant not get repository name");
        }
    };

    // 获取配置文件中的branch name
    let config_branch_name = config_data.get_config_data(repository_name, "branch");

    // 获取当前请求的分支名称
    let branch_url: &str = match request_body.pointer("/ref") {
        Some(Value::String(v)) => v,
        Some(_) | None => {
            return HttpResponse::Ok().body("Cant not get branch name");
        }
    };
    let request_branch_name = branch_url.replace("refs/heads/", "");

    // 判断分支是否符合
    if &config_branch_name[0] != "" && !config_branch_name.contains(&"*".to_string()) && !config_branch_name.contains(&request_branch_name) {
        return HttpResponse::Ok().body("Branch not match");
    }

    if signature != "" {
        let signature_bytes = match hex::decode(&signature) {
            Ok(result) => result,
            Err(error) => {
                return HttpResponse::Ok().body("Failed");
            }
        };

        let secret = &config_data.get_config_data(repository_name, "secret")[0];
        let secret = secret.as_bytes();

        let r = validate(secret, &signature_bytes, &request_body_bytes[..]);
        if !r {
            return HttpResponse::Ok().body("Signature valid failed");
        }
    }

    let mut commands = config_data.get_config_data(repository_name, "command");
    if commands.len() == 1 && &commands[0] == "" {
        match &query_info.command {
            Some(v) => {
                commands = vec![v.replace("+", " ")];
            }
            None => {
                commands = vec![];
            }
        }
    }

    for command in &commands {
        task.send(command);
    }

    HttpResponse::Ok().body("Done")
}