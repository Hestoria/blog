mod api;
mod config;

use actix_web::{middleware::Logger, App, HttpServer};
use api::user::{get_user, get_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env();
    std::env::set_var("RUST_LOGS", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(get_users).service(get_user)
    })
    .bind(format!("{}:{}", config.api_host, config.api_port))?
    .run()
    .await
}
