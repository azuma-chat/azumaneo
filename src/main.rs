use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;
use sqlx::PgPool;
use std::fs::read_to_string;

mod models;
mod routes;

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

    let server = HttpServer::new(move || {
        App::new()
            .data(AzumaState { db: db.clone() })
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(placeholder_route))
            .route("/api/info", web::to(routes::api::info::api_info))
            .route(
                "/user/register",
                web::to(routes::user::register::register_user),
            )
            .route("/user/login", web::to(routes::user::login::login_user))
            .route("/user/update", web::to(routes::user::update::update_user))
    });

    server
        .bind(&config.host_uri)
        .expect(&format!("couldn't bind to address {}", &config.host_uri))
        .run()
        .await
        .expect("couldn't run server");
}

//TODO: implement custom 404 response
