use crate::controller::default_controller;
use desire::Router;
use desire::fs::ServeFile;
use desire::fs::ServeDir;
pub fn default_routes() -> desire::Router {
  let mut router = Router::new();
  router.get("/", ServeFile::new("dist/index.html".into()));
  router.get("/assets/:file", ServeDir::new("dist".into()));
  router.get("/hello", default_controller::hello);
  router
}
