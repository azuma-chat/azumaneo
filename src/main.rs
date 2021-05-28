//! Welcome to azumaneo! We want to make it as easy as possible for possible collaborators to help us improve azuma so please don't hesitate to open a github issue or contact us by email :)

use std::fs::read_to_string;

use actix::{Actor, Addr};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use log::{trace, info};
use serde::Deserialize;
use sqlx::{migrate, ConnectOptions, PgPool};

use routes::{
    api::api_info,
    init_ws::init_ws,
    message::send_msg,
    user::{login_user, register_user, update_user},
};
use websocket::broker::Broker;

use crate::routes::textchannel::create_textchannel;
use crate::websocket::channelhandler::ChannelHandler;
use sqlx::postgres::PgConnectOptions;
use std::str::FromStr;

mod models;
mod routes;
mod websocket;

/// This route just serves as a placeholder in case a specific path is reserved for future use, but the feature is not ready for production yet.
pub fn placeholder_route(req: HttpRequest) -> HttpResponse {
    let response = format!("Welcome to Azuma!\n\nUnfortunately the requested route '{path}' is not available yet. Please come back later.", path = req.path());
    HttpResponse::NotImplemented().body(response)
}

/// 404 response route
fn not_found(req: HttpRequest) -> HttpResponse {
    let response = format!("Welcome to Azuma!\n\n\nUh ooh, we don't know what's supposed to be here... Please check if you misspelled something or used an old API documentation.\n\n{host}{path}", host = req.connection_info().host(), path = req.path());
    HttpResponse::NotFound().body(response)
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
    pretty_env_logger::init_custom_env("AZUMA_LOGLEVEL");
    let config = AzumaConfig::load("config.toml");
    // Fix for "mismatched types" error in query_as! macro: https://docs.rs/sqlx/0.4.0-beta.1/sqlx/macro.query_as.html#troubleshooting-error-mismatched-types
    let mut connection_options = PgConnectOptions::from_str(&config.db_uri)
        .expect("An error occurred while setting up the database connection")
        .application_name("azumaneo");
    connection_options.log_statements(log::LevelFilter::Off);
    let db = PgPool::connect_with(connection_options).await.unwrap();
    trace!(target: "STARTUP", "Running Migrations");
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
            // textchannel stuff
            .route("/textchannel/create", web::post().to(create_textchannel))
            // other routes
            .route("/init_ws", web::get().to(init_ws))
            .default_service(web::get().to(not_found))
    });

    info!(target: "STARTUP", "Starting azumaneo on {}", &config.host_uri);
    // start the actual http server
    server
        .bind(&config.host_uri)
        .unwrap_or_else(|_| panic!("couldn't bind to address {}", &config.host_uri))
        .run()
        .await
        .expect("couldn't run server");
}
