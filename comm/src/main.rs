use std::time::Duration;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result, error};
use sea_orm::*;
use serde::Deserialize;

use migration::{Migrator, MigratorTrait};


#[derive(Deserialize)]
struct UserInfo {
    user_id: String,
    password: String,
}

#[derive(Deserialize)]
struct UserMemberInfo {
    id: i32,
    active: i8,
    name: String,
    password: String,
    email: String,
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/home")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn sign_up(info: web::Json<UserInfo>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", info.user_id, info.password))
}

// async fn sign_in(info: web::Json<UserInfo>) -> Result<String> {
//     Ok(format!("Welcome {}, user_id {}!", info.user_id, info.password))
// }

async fn insert_member(info: web::Json<UserMemberInfo>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", info.id, info.password))
}

async fn db_connection() -> DatabaseConnection{
    //sea-orm-cli migrate up 명령어
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let mut opt = ConnectOptions::new(conn_spec.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let conn: DatabaseConnection = Database::connect(opt).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    conn
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conn = db_connection();

    HttpServer::new(|| {
        let data_info = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            // create custom error response
            error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                .into()
        });

        App::new()
           
            .service(hello)
            .service(home)
            .service(web::resource("/sign-in")
                .route(web::post().to(insert_member)))
            .service(web::resource("/sign-up")
                .route(web::post().to(sign_up)))
                .app_data(data_info.clone())

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}