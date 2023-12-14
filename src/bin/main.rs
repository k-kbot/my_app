use loco_rs::cli;
use my_app::app::App;
use migration::Migrator;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App, Migrator>().await
}
