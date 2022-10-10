use once_cell::sync::Lazy;
use std::env;
pub const ADDR: Lazy<String> = Lazy::new(|| env::var("ADDR").expect("ADDR is required"));
