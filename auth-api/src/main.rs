use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};

pub mod db;
pub mod models;
pub mod util;

#[get("/v1/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Auth endpoint")
}

#[get("/v1/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("API is healthy")
}

#[post("/v1/signup")]
async fn signup_path(info: web::Json<models::request::Request>) -> impl Responder {
    let data = util::signup::signup(info.into_inner()).await;
    HttpResponse::Ok().body(data)
}

#[post("/v1/login")]
async fn login_path(info: web::Json<models::request::Request>) -> impl Responder {
    let data = util::login::login(info.into_inner()).await;
    HttpResponse::Ok().body(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").expect("PORT must be set");

    println!("Running on http://localhost:{}", port);
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(index)
            .service(health)
            .service(signup_path)
            .service(login_path)
    })
    .bind(format!("localhost:{}", port))
    .expect("Cannot bind to given port")
    .run()
    .await
}
