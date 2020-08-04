use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse};

use serde_json::{json, Map, Value};
//use hex_literal::hex;
//use sha1::{Sha1, Digest};

use crypto::digest::Digest;
use crypto::sha1::Sha1;

#[post("/git_post_receive")]
async fn git_post_receive(req: HttpRequest, request_body: web::Json<Value>) -> HttpResponse {
    println!("REQ: {:?}", req);
    println!("REQ: {:?}", req.headers());
    let s = request_body.as_str().unwrap();
    println!("request_body:\n{}", s);


    let mut hasher = Sha1::new();
    hasher.input_str("helloaaa");
    hasher.input_str(s);
    let hex = hasher.result_str();
    println!("{:?}", hex);

//    let mut hasher = Sha1::new();
//    hasher.update(request_body.as_str().unwrap().as_bytes());
//    let mut hasher = Sha1::new();
//    hasher.update(request_body.as_str().unwrap().as_bytes());
//    let result = hasher.finalize().to_string();
//
////    assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
//    println!("{:?}", result);
//
//    let s = Sha1::digest(request_body.as_str().unwrap().as_bytes());
//    println!("{:?}", s);

    HttpResponse::Ok().body("done")
}

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s = r#"{"after":"ade570636d0b17cff5c6bfd9f1302752b95e5569","base_ref":null,"before":"40c73c6615134fd9ad07a81d5f0f1df098d9d994","commits":[{"added":[],"author":{"email":"arlicle@gmail.com","name":"edison","username":"arlicle"},"committer":{"email":"arlicle@gmail.com","name":"edison","username":"arlicle"},"distinct":true,"id":"ade570636d0b17cff5c6bfd9f1302752b95e5569","message":"Fix *","modified":["authx/views.py"],"removed":[],"timestamp":"2020-08-03T20:52:08+08:00","tree_id":"84f8ff7367cab1262995af94c1929df3aa8ad200","url":"https://github.com/arlicle/otc_exchange/commit/ade570636d0b17cff5c6bfd9f1302752b95e5569"}],"compare":"https://github.com/arlicle/otc_exchange/compare/40c73c661513...ade570636d0b","created":false,"deleted":false,"forced":false,"head_commit":{"added":[],"author":{"email":"arlicle@gmail.com","name":"edison","username":"arlicle"},"committer":{"email":"arlicle@gmail.com","name":"edison","username":"arlicle"},"distinct":true,"id":"ade570636d0b17cff5c6bfd9f1302752b95e5569","message":"Fix *","modified":["authx/views.py"],"removed":[],"timestamp":"2020-08-03T20:52:08+08:00","tree_id":"84f8ff7367cab1262995af94c1929df3aa8ad200","url":"https://github.com/arlicle/otc_exchange/commit/ade570636d0b17cff5c6bfd9f1302752b95e5569"},"pusher":{"email":"arlicle@gmail.com","name":"arlicle"},"ref":"refs/heads/master","repository":{"archive_url":"https://api.github.com/repos/arlicle/otc_exchange/{archive_format}{/ref}","archived":false,"assignees_url":"https://api.github.com/repos/arlicle/otc_exchange/assignees{/user}","blobs_url":"https://api.github.com/repos/arlicle/otc_exchange/git/blobs{/sha}","branches_url":"https://api.github.com/repos/arlicle/otc_exchange/branches{/branch}","clone_url":"https://github.com/arlicle/otc_exchange.git","collaborators_url":"https://api.github.com/repos/arlicle/otc_exchange/collaborators{/collaborator}","comments_url":"https://api.github.com/repos/arlicle/otc_exchange/comments{/number}","commits_url":"https://api.github.com/repos/arlicle/otc_exchange/commits{/sha}","compare_url":"https://api.github.com/repos/arlicle/otc_exchange/compare/{base}...{head}","contents_url":"https://api.github.com/repos/arlicle/otc_exchange/contents/{+path}","contributors_url":"https://api.github.com/repos/arlicle/otc_exchange/contributors","created_at":1595549360,"default_branch":"master","deployments_url":"https://api.github.com/repos/arlicle/otc_exchange/deployments","description":null,"disabled":false,"downloads_url":"https://api.github.com/repos/arlicle/otc_exchange/downloads","events_url":"https://api.github.com/repos/arlicle/otc_exchange/events","fork":false,"forks":0,"forks_count":0,"forks_url":"https://api.github.com/repos/arlicle/otc_exchange/forks","full_name":"arlicle/otc_exchange","git_commits_url":"https://api.github.com/repos/arlicle/otc_exchange/git/commits{/sha}","git_refs_url":"https://api.github.com/repos/arlicle/otc_exchange/git/refs{/sha}","git_tags_url":"https://api.github.com/repos/arlicle/otc_exchange/git/tags{/sha}","git_url":"git://github.com/arlicle/otc_exchange.git","has_downloads":true,"has_issues":true,"has_pages":false,"has_projects":true,"has_wiki":true,"homepage":null,"hooks_url":"https://api.github.com/repos/arlicle/otc_exchange/hooks","html_url":"https://github.com/arlicle/otc_exchange","id":282082665,"issue_comment_url":"https://api.github.com/repos/arlicle/otc_exchange/issues/comments{/number}","issue_events_url":"https://api.github.com/repos/arlicle/otc_exchange/issues/events{/number}","issues_url":"https://api.github.com/repos/arlicle/otc_exchange/issues{/number}","keys_url":"https://api.github.com/repos/arlicle/otc_exchange/keys{/key_id}","labels_url":"https://api.github.com/repos/arlicle/otc_exchange/labels{/name}","language":"Python","languages_url":"https://api.github.com/repos/arlicle/otc_exchange/languages","license":null,"master_branch":"master","merges_url":"https://api.github.com/repos/arlicle/otc_exchange/merges","milestones_url":"https://api.github.com/repos/arlicle/otc_exchange/milestones{/number}","mirror_url":null,"name":"otc_exchange","node_id":"MDEwOlJlcG9zaXRvcnkyODIwODI2NjU=","notifications_url":"https://api.github.com/repos/arlicle/otc_exchange/notifications{?since,all,participating}","open_issues":0,"open_issues_count":0,"owner":{"avatar_url":"https://avatars3.githubusercontent.com/u/153773?v=4","email":"arlicle@gmail.com","events_url":"https://api.github.com/users/arlicle/events{/privacy}","followers_url":"https://api.github.com/users/arlicle/followers","following_url":"https://api.github.com/users/arlicle/following{/other_user}","gists_url":"https://api.github.com/users/arlicle/gists{/gist_id}","gravatar_id":"","html_url":"https://github.com/arlicle","id":153773,"login":"arlicle","name":"arlicle","node_id":"MDQ6VXNlcjE1Mzc3Mw==","organizations_url":"https://api.github.com/users/arlicle/orgs","received_events_url":"https://api.github.com/users/arlicle/received_events","repos_url":"https://api.github.com/users/arlicle/repos","site_admin":false,"starred_url":"https://api.github.com/users/arlicle/starred{/owner}{/repo}","subscriptions_url":"https://api.github.com/users/arlicle/subscriptions","type":"User","url":"https://api.github.com/users/arlicle"},"private":true,"pulls_url":"https://api.github.com/repos/arlicle/otc_exchange/pulls{/number}","pushed_at":1596459145,"releases_url":"https://api.github.com/repos/arlicle/otc_exchange/releases{/id}","size":132,"ssh_url":"git@github.com:arlicle/otc_exchange.git","stargazers":0,"stargazers_count":0,"stargazers_url":"https://api.github.com/repos/arlicle/otc_exchange/stargazers","statuses_url":"https://api.github.com/repos/arlicle/otc_exchange/statuses/{sha}","subscribers_url":"https://api.github.com/repos/arlicle/otc_exchange/subscribers","subscription_url":"https://api.github.com/repos/arlicle/otc_exchange/subscription","svn_url":"https://github.com/arlicle/otc_exchange","tags_url":"https://api.github.com/repos/arlicle/otc_exchange/tags","teams_url":"https://api.github.com/repos/arlicle/otc_exchange/teams","trees_url":"https://api.github.com/repos/arlicle/otc_exchange/git/trees{/sha}","updated_at":"2020-08-03T12:25:54Z","url":"https://github.com/arlicle/otc_exchange","watchers":0,"watchers_count":0},"sender":{"avatar_url":"https://avatars3.githubusercontent.com/u/153773?v=4","events_url":"https://api.github.com/users/arlicle/events{/privacy}","followers_url":"https://api.github.com/users/arlicle/followers","following_url":"https://api.github.com/users/arlicle/following{/other_user}","gists_url":"https://api.github.com/users/arlicle/gists{/gist_id}","gravatar_id":"","html_url":"https://github.com/arlicle","id":153773,"login":"arlicle","node_id":"MDQ6VXNlcjE1Mzc3Mw==","organizations_url":"https://api.github.com/users/arlicle/orgs","received_events_url":"https://api.github.com/users/arlicle/received_events","repos_url":"https://api.github.com/users/arlicle/repos","site_admin":false,"starred_url":"https://api.github.com/users/arlicle/starred{/owner}{/repo}","subscriptions_url":"https://api.github.com/users/arlicle/subscriptions","type":"User","url":"https://api.github.com/users/arlicle"}}"#;
    let mut hasher = Sha1::new();
    hasher.input_str("helloaaa");
    hasher.input_str(s);
    let hex = hasher.result_str();
    println!("{:?}", hex);


    HttpServer::new(|| App::new()
        .service(git_post_receive)
        .service(index))
        .bind("0.0.0.0:9005")?
        .run()
        .await
}


