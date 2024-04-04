use std::time::Instant;
use axum::routing::get;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tower_http::trace::TraceLayer;

mod routes;
mod error_handling;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let start_time = Instant::now();
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").expect("Klarte ikke å lese DATABASE_URL fra miljøvariabler");

    info!("Applikasjonen starter...");

    let db = PgPoolOptions::new()
        .max_connections(128)
        .connect(&db_url)
        .await
        .expect("Klarte ikke å koble til databasen");

    let app = axum::Router::new()
        .route("/users", get(routes::users::get))
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
