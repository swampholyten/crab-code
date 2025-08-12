use server::app::run;
use server::errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}
