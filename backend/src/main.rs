mod data;
mod model;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use routes::geocoding::get_locations;
use routes::ping::ping;
use routes::weather::get_weather;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(ping)
            .service(get_locations)
            .service(get_weather)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
