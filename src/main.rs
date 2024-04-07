use std::time::Instant;

use axum::routing::{get, post};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::settings::get_setting;

mod routes;
mod types;
mod utils;
mod settings;
mod db;

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

    let app = axum::Router::new()
        .route("/users", get(routes::users::get))
        .route("/users/register", post(routes::users::register::post))
        .route("/users/register/confirm", get(routes::users::confirm_registration::get))
        .route("/users/regenerate-token", post(routes::users::generate_new_token::post))
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;
    info!(
        "Applikasjon klar på {:.2}s - lytter på {}",
        start_time.elapsed().as_secs_f32(),
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await?;

    Ok(())
}
