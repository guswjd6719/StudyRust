
use actix_web::{Responder, Error, get, web, post, App, HttpServer, Result, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
// struct사용해서 url 뒤에 ? 형태로 넘어오는 파라미터 처리
#[get("/query")]
async fn query(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

/// deserialize `Info` from request's body
/// json 형태로 리퀘스트
#[post("/json")]
async fn json(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32 - 데이터 타입에 맞게 전송해야 함
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn paths(req: HttpRequest) -> Result<String> {
    dbg!(&req);
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id: {}!", name, userid))
}

use futures::{future::ok, stream::once};

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));
// Response body로 비동기 처리
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[get("/youtube")]
async fn youtube(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", ["oHg5SJYRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

    url.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
            .service(paths)
            .service(query)
            .service(json)
            .service(stream)
            .service(
                //Scoping Routes
                web::scope("/dispatch")
                    .service(show_users)
                    .service(user_detail),
            )
            .service(youtube)
            .external_resource("youtube", "https://youtube.com/watch/{video_id}")
        }  )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}