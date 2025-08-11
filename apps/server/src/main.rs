use server::{
    common::{config::Config, state::State},
    server::create_router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    setup_tracing();

    let config = Config::from_env()?;
    let state = State::new(config.clone());

    let app = create_router(state);

    let addr = format!("{}:{}", config.service_host, config.service_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server running at {addr}");

    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_tracing() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE),
        )
        .init();
}
