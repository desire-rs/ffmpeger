use crate::controller::default_controller;
use desire::fs::ServeDir;
use desire::fs::ServeFile;
use desire::Router;
pub fn default_routes() -> desire::Router {
  let mut router = Router::new();
  router.get("/", ServeFile::new("web/dist/index.html".into()));
  router.get("/assets/:file", ServeDir::new("web/dist".into()));
  router.get("/liveness", default_controller::liveness);
  router.get("/hello", default_controller::hello);
  router
}
