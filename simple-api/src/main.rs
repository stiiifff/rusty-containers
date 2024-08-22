use std::env;

use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg};
use dotenv::dotenv;
use log::{debug, info};

const DEFAULT_PORT: &str = "8080";

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body(format!("{} v{}",  crate_name!(), crate_version!()))
}

#[get("/healthz")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Sets a port")
                .takes_value(true),
        )
        .get_matches();

    let port: u16 = matches
        .value_of("port")
        .map(|s| s.to_owned())
        .or_else(|| env::var("PORT").ok())
        .unwrap_or_else(|| DEFAULT_PORT.to_owned()) // Default port
        .parse()
        .expect("can't parse PORT variable");

    debug!("Starting server ...");
    
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(root)
            .service(health)
    })
    .bind(("0.0.0.0", port))?
    .run();

    info!("Server '{}' is running on port {}", crate_name!(), port);

    server.await
}
