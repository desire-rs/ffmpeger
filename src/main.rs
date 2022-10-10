#[macro_use]
extern crate tracing;

mod config;
mod controller;
mod error;
mod routes;
mod types;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> types::AnyResult<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  let args: Vec<String> = std::env::args().collect();
  let env_name = args.get(1).expect("env name is required");
  let env_file = format!("env/{}.env", env_name);
  dotenv::from_filename(&env_file).ok();

  let app = routes::default_routes();
  let svc = desire::new(config::ADDR.as_str());
  svc.run(app).await?;
  info!("Hello, world!");
  Ok(())
}
