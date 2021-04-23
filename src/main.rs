//! Welcome to azumaneo! We want to make it as easy as possible for possible collaborators to help us improve azuma so please don't hesitate to open a github issue or contact us by email :)

use std::fs::read_to_string;

use actix::{Actor, Addr};
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;
use sqlx::{migrate, PgPool};

use routes::{
    api::api_info,
    init_ws::init_ws,
    user::{login_user, register_user, update_user},
};

use crate::routes::message::send_msg;
use crate::websocket::channelhandler::ChannelHandler;
use crate::websocket::chatserver::ChatServer;

mod models;
mod routes;
mod websocket;

//define placeholder route
/// This route just serves as a placeholder in case a specific path is reserved for future use, but the feature is not ready for production yet.
pub fn placeholder_route(req: HttpRequest) -> HttpResponse {
    let response = format!("Welcome to Azuma!\nUnfortunately the requested route '{path}' is not available yet. Please come back later.", path = req.path());
    HttpResponse::NotImplemented().body(response)
}

/// This constant defines the awsp version and makes sure that there are no crashes on the client side.
/// It makes it easier for the client to know if the server or the client is out of date and who should update.
const AWSP_VERSION: u8 = 1;

/// The AzumaConfig holds every value defined in the config.toml file for internal use in the server
#[derive(Deserialize)]
struct AzumaConfig {
    host_uri: String,
    db_uri: String,
}

/// This struct holds all the values that are not user-configurable
#[derive(Clone, Debug)]
pub struct AzumaConstants {
    /// This holds the current version of the awsp websocket protocol
    awsp_version: u8,
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

/// The application state holds every data needed by the http routes
#[derive(Debug, Clone)]
pub struct AzumaState {
    /// The [`PgPool`] enables database connection in all the http routes
    pub db: PgPool,
    /// This enables the
    pub srv: Addr<ChatServer>,
    pub channelhandler: Addr<ChannelHandler>,
    pub constants: AzumaConstants,
}

/// Nothing to really say here, in the main function we start up all the internals
#[actix_web::main]
async fn main() {
    pretty_env_logger::init();
    let config = AzumaConfig::load("config.toml");
    let db = PgPool::connect(&config.db_uri).await.unwrap();
    let constants = AzumaConstants {
        awsp_version: AWSP_VERSION,
    };
    migrate!("./migrations/")
        .run(&db)
        .await
        .expect("couldn't run database migrations");

    // start the http server, set the http routes and state data
    let server = HttpServer::new(move || {
        App::new()
            .data(AzumaState {
                db: db.clone(),
                srv: ChatServer::new(db.clone()).start(),
                channelhandler: ChannelHandler::new(db.clone()).start(),
                constants: constants.clone(),
            })
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
