use crate::libs::{get_pool, get_redis_client};

use desire::Middleware;
use desire::Request;
use desire::Result;
use std::time::Instant;
pub struct Logger;

#[async_trait::async_trait]
impl Middleware for Logger {
  async fn handle(&self, req: Request, next: desire::Next<'_>) -> Result {
    let start = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let res = next.run(req).await?;
    println!(
      "{} {} {} {}ms",
      method,
      path,
      res.status().to_string(),
      start.elapsed().as_millis()
    );
    Ok(res)
  }
}

pub struct DB;
#[async_trait::async_trait]
impl Middleware for DB {
  async fn handle(&self, mut req: Request, next: desire::Next<'_>) -> Result {
    let pool = get_pool().await?;
    let client = get_redis_client().await?;
    req.inner.extensions_mut().insert(pool);
    req.inner.extensions_mut().insert(client);
    next.run(req).await
  }
}
