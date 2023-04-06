use async_trait::async_trait;
use db::station_picture_variant::{StationPictureVariant, StationPictureVariantFormat};
use db::Model;
use hyper::header::{CACHE_CONTROL, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, StatusCode};
use mongodb::bson::doc;
use prex::{handler::Handler, Request, Response};

use crate::error::ApiError;

#[derive(Debug, Clone, Copy)]
pub enum StationPicHandler {
  Webp(f64),
  Png(f64),
  Source,
}

#[async_trait]
impl Handler for StationPicHandler {
  async fn call(&self, req: Request, _next: prex::Next) -> Response {
    let id = req.param("picture").unwrap();

    let filter = match *self {
      Self::Webp(size) => doc! {
        StationPictureVariant::KEY_PICTURE_ID: id,
        StationPictureVariant::KEY_FORMAT: StationPictureVariantFormat::Webp,
        StationPictureVariant::KEY_SIZE: size,
      },

      Self::Png(size) => doc! {
        StationPictureVariant::KEY_PICTURE_ID: id,
        StationPictureVariant::KEY_FORMAT: StationPictureVariantFormat::Png,
        StationPictureVariant::KEY_SIZE: size,
      },

      Self::Source => doc! {
        StationPictureVariant::KEY_PICTURE_ID: id,
        StationPictureVariant::KEY_FORMAT: StationPictureVariantFormat::Source,
      },
    };

    match StationPictureVariant::get(filter).await {
      Err(e) => ApiError::from(e).into_json_response(),
      Ok(r) => match r {
        None => ApiError::ResourceNotFound {}.into_json_response(),
        Some(doc) => {
          let mut res = Response::new(StatusCode::OK);

          match HeaderValue::from_str(&doc.content_type) {
            Err(_) => res.headers_mut().append(
              CONTENT_TYPE,
              HeaderValue::from_static("application/octet-stream"),
            ),
            Ok(value) => res.headers_mut().append(CONTENT_TYPE, value),
          };

          res.headers_mut().append(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"), // 365 days
          );

          let body = Body::from(doc.data);

          *res.body_mut() = body;

          res
        }
      },
    }
  }
}
