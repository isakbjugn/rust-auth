use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;

use axum::routing::{get, post};
use sqlx::postgres::PgPoolOptions;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::settings::settings;

mod db;
mod extractors;
mod routes;
mod settings;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let start_time = Instant::now();
    tracing_subscriber::fmt::init();

    info!("Applikasjonen starter...");

    let db = PgPoolOptions::new()
        .max_connections(128)
        .connect(&settings().database_url)
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
        .route("/login", post(routes::login::post))
        .route("/login/local", post(routes::login::local::post))
        .route("/users", get(routes::users::get))
        .route("/users/all", get(routes::users::all::get))
        .route("/users/register", post(routes::users::register::post))
        .route("/users/register/confirm", post(routes::users::confirm_registration::post))
        .route("/users/regenerate-token", post(routes::users::generate_new_token::post))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .with_state(db);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), settings().port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(
        "Applikasjon klar på {:.2}s - lytter på {}",
        start_time.elapsed().as_secs_f32(),
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await?;

    Ok(())
}
