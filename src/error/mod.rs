use desire::{IntoResponse, Response, Result};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
  #[error("Anyhow::Error")]
  AnyError(#[from] anyhow::Error),
  #[error("Desire::Error")]
  DesireError(#[from] desire::Error),
  #[error("Desire::Error")]
  UuidError(#[from] uuid::Error),
  #[error("io error")]
  IoError(#[from] std::io::Error),
  #[error("sqlx::Error {0:?}")]
  SqlxError(#[from] sqlx::Error),
  #[error("redis::Error {0:?}")]
  RedisError(#[from] redis::RedisError),
  #[error("serde_json::Error")]
  JsonError(#[from] serde_json::Error),
  #[error("unwrap `{0}` is not none")]
  OptionError(String),
}

impl IntoResponse for Error {
  fn into_response(self) -> Result {
    let val = self.to_string();
    Response::with_status(500, val)
  }
}

pub fn option_error(msg: &str) -> Error {
  Error::OptionError(msg.to_string())
}
