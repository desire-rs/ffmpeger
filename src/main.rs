#[macro_use]
extern crate tracing;

mod config;
mod controller;
mod error;
mod libs;
mod middleware;
mod routes;
mod schema;
mod service;
mod types;
mod utils;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use config::{ADDR, ENV_NAME};
use error::Error;
use types::ApiPageResult;
use types::ApiResult;
use types::PageData;
use types::Resp;
#[tokio::main]
async fn main() -> types::AnyResult<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  let arguments: Vec<String> = std::env::args().collect();
  let env_name = arguments.get(1).expect("env name must be provided");
  let env_file = format!("env/{}.env", env_name);
  dotenv::from_filename(env_file).ok();
  info!("ENV_NAME: {}", ENV_NAME.as_str());
  let app = routes::default_routes();
  let svc = desire::new(ADDR.as_str());
  svc.run(app).await?;
  Ok(())
}
