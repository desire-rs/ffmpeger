use crate::schema::task_schema::Task;
use crate::service::ffmpeg_service;
use crate::Resp;
use crate::ApiResult;
use desire::Request;
pub async fn m3u8(mut req: Request) -> ApiResult<Task> {
  let task= req.body::<Task>().await?;
  info!("task {:#?}", task);
  let json = task.clone();
  let _handle = tokio::task::spawn(async move {
    let res = ffmpeg_service::m3u8(task).await;
    println!("res {:#?}", res);
  });
  Ok(Resp::data(json))
}
