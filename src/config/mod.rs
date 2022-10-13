use once_cell::sync::Lazy;
use std::env;
pub static ENV_NAME: Lazy<String> =
  Lazy::new(|| env::var("ENV_NAME").expect("ENV_NAME must be set"));
pub static DATABASE_URI: Lazy<String> =
  Lazy::new(|| env::var("DATABASE_URI").expect("DATABASE_URI must be set"));
pub static REDIS_URI: Lazy<String> =
  Lazy::new(|| env::var("REDIS_URI").expect("REDIS_URI must be set"));
pub static ADDR: Lazy<String> = Lazy::new(|| env::var("ADDR").expect("ADDR must be set"));

pub const TASK_HASH: &'static str = "FFMPEGER_TASK_HASH";
pub const TASK_LIMIT: i64 = 4;
