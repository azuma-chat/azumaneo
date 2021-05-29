//! Welcome to azumaneo!
//! If you run into any problems, don't hesitate to create an issue on GitHub.
//! Contributions are welcome, just take a look at currently open issues or create a new one.

use std::fs::read_to_string;

use actix::{Actor, Addr};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};
use log::info;
use serde::Deserialize;
use sqlx::{migrate, PgPool};

use routes::{
    api::api_info,
    init_ws::init_ws,
    message::send_msg,
    user::{login_user, register_user, update_user},
};
use websocket::broker::Broker;

use crate::models::error::AzumaError;
use crate::routes::textchannel::create_textchannel;

mod models;
mod routes;
mod websocket;

/// 404 response route
async fn not_found(_req: HttpRequest) -> Result<HttpResponse, AzumaError> {
    Err(AzumaError::NotFound)
}

/// The AzumaConfig is loaded on startup and made available in the Actix-Web data
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

#[derive(Clone)]
pub struct AzumaState {
    pub db: PgPool,
    pub broker: Addr<Broker>,
}

#[actix_web::main]
async fn main() {
    pretty_env_logger::init_timed();
    let config = AzumaConfig::load("config.toml");

    let db = PgPool::connect(&config.db_uri).await.unwrap();
    migrate!("./migrations/")
        .run(&db)
        .await
        .expect("couldn't run database migrations");

    let broker = Broker::new().start();

    let state = AzumaState {
        db: db.clone(),
        broker: broker.clone(),
    };

    let server = HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(Logger::default())
            // general API routes
            .route("/", web::get().to(api_info))
            .route("/init_ws", web::get().to(init_ws))
            // user routes
            .route("/user/register", web::post().to(register_user))
            .route("/user/login", web::post().to(login_user))
            .route("/user/update", web::patch().to(update_user))
            // message routes
            .route("/message/send", web::post().to(send_msg))
            // textchannel stuff
            .route("/textchannel/create", web::post().to(create_textchannel))
            // custom 404 response
            .default_service(web::route().to(not_found))
    });

    info!("Starting azumaneo on {}", &config.host_uri);
    server
        .bind(&config.host_uri)
        .expect(&format!("couldn't bind to address {}", &config.host_uri))
        .run()
        .await
        .expect("couldn't run server");
}
