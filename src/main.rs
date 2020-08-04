use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::web::{Bytes, post, Query};
use serde_json::{json, Map, Value};
use sha1::Sha1;
use hmac::{Hmac, Mac, NewMac};
use hex;

type HmacSha1 = Hmac<Sha1>;

pub fn validate(secret: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let mut hmac = HmacSha1::new_varkey(secret).unwrap();
    hmac.update(message);

    let result = hmac.finalize();
    let code_bytes = result.into_bytes();

    println!("signature: {:?}", signature);
    println!("code_bytes: {:?}", code_bytes);
//    hmac.verify(signature).is_ok()
    true
}



#[post("/git_post_receive")]
async fn git_post_receive(req: HttpRequest, bytes: Bytes) -> HttpResponse {
    println!("REQ: {:?}", req);
    println!("REQ: {:?}", req.headers());
    println!("REQ: {:?}", bytes);

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


    let secret = "helloaaa".as_bytes();
    let r = validate(secret, &signature_bytes, &bytes[..]);
    println!("result {}", r);
    HttpResponse::Ok().body("Done")
}

#[get("/")]
async fn index(req: HttpRequest, bytes: Bytes) -> HttpResponse {




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


