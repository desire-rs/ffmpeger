use crate::config::{CACHE_REPORT, CACHE_REPORT_HASH, TASK_HASH};
use crate::libs::get_redis_client;
use crate::schema::task_schema::{Task, TaskReport};
use crate::types::AnyResult;
use redis::AsyncCommands;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
pub async fn m3u8(payload: Task) -> AnyResult<TaskReport> {
  let client = get_redis_client().await?;
  let mut redis = client.get_async_connection().await?;
  let mut args: Vec<&str> = Vec::new();
  if let Some(user_agent) = &payload.user_agent {
    args.push("-user_agent");
    args.push(user_agent.as_str());
  }
  if let Some(referer) = &payload.referer {
    args.push("-referer");
    args.push(referer.as_str());
  }
  if let Some(cookies) = &payload.cookies {
    args.push("-cookies");
    args.push(cookies.as_str());
  }
  if let Some(headers) = &payload.headers {
    args.push("-headers");
    args.push(headers.as_str());
  }
  args.push("-i");
  args.push(&payload.url);
  args.push("-y");
  args.push("-c");
  args.push("copy");
  args.push(&payload.storage_path);
  println!("{:?}", args);
  let mut cmd = Command::new("ffmpeg")
    .args(&args)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;
  let stderr = BufReader::new(cmd.stderr.take().unwrap());
  let mut lines = stderr.lines();
  while let Some(line) = lines.next_line().await? {
    println!("{}", line);
  }
  let result = cmd.wait().await.expect("msg");
  let success = result.success();
  let result = format!("result {:?} success {}", result, success);
  println!("result: {}", result);
  redis.hdel(TASK_HASH, &payload.id.to_string()).await?;
  let mut report: TaskReport =
    TaskReport::new(payload.id, payload.title, payload.url, payload.storage_path);
  report.status = Some(result);
  if CACHE_REPORT {
    redis
      .hset_nx(
        CACHE_REPORT_HASH,
        &report.title,
        serde_json::to_string(&report)?,
      )
      .await?;
  }
  Ok(report)
}
