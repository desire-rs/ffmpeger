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
pub const CACHE_REPORT: bool = true;
pub const SLACK_REPORT: bool = true;
pub const CACHE_REPORT_HASH: &'static str = "FFMPEGER_REPORT_HASH";
pub const SLACK_WEBHOOK_URL :&'static str = "https://hooks.slack.com/services/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";