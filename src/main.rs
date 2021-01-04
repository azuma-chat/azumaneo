mod models;
mod routes;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use routes::{
    api::api_info,
    user::{login_user, register_user, update_user},
};
use serde::Deserialize;
use sqlx::{migrate, PgPool};
use std::fs::read_to_string;

//define placeholder route
pub fn placeholder_route(req: HttpRequest) -> HttpResponse {
    let response = format!("Welcome to Azuma!\nUnfortunately the requested route '{path}' is not available yet. Please come back later.", path = req.path());
    HttpResponse::NotImplemented().body(response)
}

#[derive(Deserialize)]
struct AzumaConfig {
    host_uri: String,
    db_uri: String,
}

impl AzumaConfig {
    fn load(path: &str) -> Self {
        let config_string = read_to_string(path).expect("couldn't load config from provided path");
        let config: AzumaConfig =
            toml::from_str(&config_string).expect("couldn't deserialize config");
        config
    }
}

pub struct AzumaState {
    pub db: PgPool,
}

#[actix_web::main]
async fn main() {
    pretty_env_logger::init();
    let config = AzumaConfig::load("config.toml");
    let db = PgPool::connect(&config.db_uri).await.unwrap();
    migrate!("./migrations/")
        .run(&db)
        .await
        .expect("couldn't run database migrations");

    let server = HttpServer::new(move || {
        App::new()
            .data(AzumaState { db: db.clone() })
            .wrap(middleware::Logger::default())
            // general API routes
            .route("/", web::get().to(api_info))
            // user routes
            .route("/user/register", web::post().to(register_user))
            .route("/user/login", web::post().to(login_user))
            .route("/user/update", web::patch().to(update_user))
    });

    server
        .bind(&config.host_uri)
        .expect(&format!("couldn't bind to address {}", &config.host_uri))
        .run()
        .await
        .expect("couldn't run server");
}

//TODO: implement custom 404 response
