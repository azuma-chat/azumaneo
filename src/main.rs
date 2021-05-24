//! Welcome to azumaneo! We want to make it as easy as possible for possible collaborators to help us improve azuma so please don't hesitate to open a github issue or contact us by email :)

use std::fs::read_to_string;

use actix::{Actor, Addr};
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;
use sqlx::{migrate, PgPool};

use routes::{
    api::api_info,
    init_ws::init_ws,
    message::send_msg,
    user::{login_user, register_user, update_user},
};
use websocket::broker::Broker;

use crate::websocket::channelhandler::ChannelHandler;

mod models;
mod routes;
mod websocket;

/// This route just serves as a placeholder in case a specific path is reserved for future use, but the feature is not ready for production yet.
pub fn placeholder_route(req: HttpRequest) -> HttpResponse {
    let response = format!("Welcome to Azuma!\nUnfortunately the requested route '{path}' is not available yet. Please come back later.", path = req.path());
    HttpResponse::NotImplemented().body(response)
}

/// The AzumaConfig holds every value defined in the config.toml file for internal use in the server
#[derive(Deserialize)]
struct AzumaConfig {
    host_uri: String,
    db_uri: String,
}

impl AzumaConfig {
    /// Load up the `config.toml` file, parse it and return a [`AzumaConfig`] struct
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
    pub channelhandler: Addr<ChannelHandler>,
}

#[actix_web::main]
async fn main() {
    pretty_env_logger::init();
    let config = AzumaConfig::load("config.toml");
    // Fix for "mismatched types" error in query_as! macro: https://docs.rs/sqlx/0.4.0-beta.1/sqlx/macro.query_as.html#troubleshooting-error-mismatched-types
    let db = PgPool::connect(&config.db_uri).await.unwrap();
    migrate!("./migrations/")
        .run(&db)
        .await
        .expect("couldn't run database migrations");

    let broker = Broker::new().start();
    let channelhandler = ChannelHandler::new(db.clone()).start();

    let state = AzumaState {
        db: db.clone(),
        broker: broker.clone(),
        channelhandler: channelhandler.clone(),
    };
    // start the http server, set the http routes and state data
    let server = HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            // general API routes
            .route("/", web::get().to(api_info))
            // user routes
            .route("/user/register", web::post().to(register_user))
            .route("/user/login", web::post().to(login_user))
            .route("/user/update", web::patch().to(update_user))
            .route(
                "/user/onlinestatus",
                web::post().to(routes::onlinestatus::update_onlinestatus),
            )
            // message related routes
            .route("/message/send", web::post().to(send_msg))
            // other routes
            .route("/init_ws", web::get().to(init_ws))
    });

    println!("Starting azumaneo on {}", &config.host_uri);
    // start the actual http server
    server
        .bind(&config.host_uri)
        .unwrap_or_else(|_| panic!("couldn't bind to address {}", &config.host_uri))
        .run()
        .await
        .expect("couldn't run server");
}

//TODO: implement custom 404 response
