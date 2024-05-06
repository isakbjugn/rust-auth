use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;

use axum::routing::{get, post};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::settings::get_setting;

mod db;
mod extractors;
mod routes;
mod settings;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let start_time = Instant::now();
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = get_setting("DATABASE_URL");

    info!("Applikasjonen starter...");

    let db = PgPoolOptions::new()
        .max_connections(128)
        .connect(&db_url)
        .await
        .expect("Klarte ikke å koble til databasen");

    /*
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Klarte ikke å kjøre migreringer");
     */

    let app = axum::Router::new()
        .route("/", get(routes::get))
        .route("/users", get(routes::users::get))
        .route("/users/all", get(routes::users::all::get))
        .route("/users/login", post(routes::users::login::post))
        .route("/users/register", post(routes::users::register::post))
        .route("/users/register/confirm", get(routes::users::confirm_registration::get))
        .route("/users/regenerate-token", post(routes::users::generate_new_token::post))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .with_state(db);

    let port = std::env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(
        "Applikasjon klar på {:.2}s - lytter på {}",
        start_time.elapsed().as_secs_f32(),
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await?;

    Ok(())
}
