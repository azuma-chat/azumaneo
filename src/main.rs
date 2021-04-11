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

use crate::routes::send_msg::send_msg;
use crate::websocket::channelhandler::ChannelHandler;
use crate::websocket::chatserver::ChatServer;

mod models;
mod routes;
mod websocket;

//define placeholder route
pub fn placeholder_route(req: HttpRequest) -> HttpResponse {
    let response = format!("Welcome to Azuma!\nUnfortunately the requested route '{path}' is not available yet. Please come back later.", path = req.path());
    HttpResponse::NotImplemented().body(response)
}
const AWSP_VERSION: u8 = 1;

#[derive(Deserialize)]
struct AzumaConfig {
    host_uri: String,
    db_uri: String,
}

#[derive(Clone, Debug)]
pub struct AzumaConstants {
    awsp_version: u8,
}

impl AzumaConfig {
    fn load(path: &str) -> Self {
        let config_string = read_to_string(path).expect("couldn't load config from provided path");
        let config: AzumaConfig =
            toml::from_str(&config_string).expect("couldn't deserialize config");
        config
    }
}
#[derive(Debug, Clone)]
pub struct AzumaState {
    pub db: PgPool,
    pub srv: Addr<ChatServer>,
    pub channelhandler: Addr<ChannelHandler>,
    pub constants: AzumaConstants,
}

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
            .route("/init_ws", web::get().to(init_ws))
            .route("/message/send", web::post().to(send_msg))
    });

    server
        .bind(&config.host_uri)
        .unwrap_or_else(|_| panic!("couldn't bind to address {}", &config.host_uri))
        .run()
        .await
        .expect("couldn't run server");
}

//TODO: implement custom 404 response
