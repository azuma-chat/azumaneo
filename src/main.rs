use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use log::info;
use serde::Deserialize;
use sqlx::PgPool;
use std::fs::read_to_string;
use tokio::{signal, sync::oneshot, task};

mod models;
mod routes;
mod util;

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

#[actix_rt::main]
async fn main() {
    let config = AzumaConfig::load("config.toml");
    let db = PgPool::connect(&config.db_uri).await.unwrap();

    let (tx, _rx) = oneshot::channel();
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

    let server = server
        .bind(&config.host_uri)
        .expect(&format!("couldn't bind to address {}", &config.host_uri));

    task::spawn(server.run());
    info!("Listening on {}", &config.host_uri);

    signal::ctrl_c()
        .await
        .expect("Couldn't listen to CTRL-C signal");
    let _ = tx.send(());
}

//TODO: implement custom 404 response
