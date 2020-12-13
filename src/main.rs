use std::net::SocketAddr;
use tokio::{signal, sync::oneshot, task};
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use log::info;

mod routes;


pub fn placeholder_route(req: HttpRequest) -> HttpResponse {
    let response = format!("Welcome to Azuma!\nUnfortunately the requested route '{path}' is not available yet. Please come back later.", path = req.path());
    HttpResponse::Ok().body(response)
}

#[actix_rt::main]
async fn main() {
    /* let listen_addr: SocketAddr = env::var("AZUMA_HOST")
         .expect("Environment variable AZUMA_HOST not found")
         .parse()
         .expect("Couldn't parse AZUMA_HOST");*/
    let listen_addr: SocketAddr = SocketAddr::new("0.0.0.0".parse().unwrap(), 8080);

    let (tx, rx) = oneshot::channel();
    let mut server = HttpServer::new(move || App::new()
        .wrap(middleware::Logger::default())
        .route("/", web::get().to(placeholder_route))
        .route("/api/info", web::to(routes::api::info::api_info))
        .route("/user/register", web::to(routes::user::register::register_user))
        .route("/user/login", web::to(routes::user::login::login_user))
    );


    let server = server.bind(listen_addr).expect(&*format!("cannot bind to address {}", listen_addr));


    task::spawn(server.run());
    info!("Listening on {}", listen_addr);


    signal::ctrl_c()
        .await
        .expect("Couldn't listen to CTRL-C signal");
    let _ = tx.send(());
}
