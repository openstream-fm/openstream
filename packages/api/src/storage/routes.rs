use db::station_picture::StationPicture;
use prex::router::builder::Builder;

use crate::routes as api_routes;

pub fn router() -> Builder {
  let mut router = prex::prex();

  router.get(
    "/stations/:station/files/:file/stream",
    api_routes::stations::files::id::stream::Handler {},
  );

  for size in StationPicture::WEBP_SIZES {
    let handler = api_routes::station_pics::StationPicHandler::Webp(size);
    let path = format!("/station-pictures/webp/{}/:picture", size as u64);
    router.get(path, handler);
  }

  for size in StationPicture::PNG_SIZES {
    let handler = api_routes::station_pics::StationPicHandler::Webp(size);
    let path = format!("/station-pictures/png/{}/:picture", size as u64);
    router.get(path, handler);
  }

  router.get(
    "/station-pictures/src/:picture",
    api_routes::station_pics::StationPicHandler::Source,
  );

  router
}
