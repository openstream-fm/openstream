use prex::router::builder::Builder;

use crate::routes as api_routes;

pub fn router() -> Builder {
  let mut router = prex::prex();

  router.get(
    "/stations/:station/files/:file/stream",
    api_routes::stations::files::id::stream::Handler {},
  );

  router
}
