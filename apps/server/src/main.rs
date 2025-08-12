use server::{
    app::{create_router, setup_tracing},
    common::{
        config::{self, Config},
        state::AppState,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    setup_tracing();

    let config = config::load();
    let state = todo!();

    let app = create_router(state);

    let addr = format!("{}:{}", config.service_host, config.service_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server running at {addr}");

    axum::serve(listener, app).await?;

    Ok(())
}
