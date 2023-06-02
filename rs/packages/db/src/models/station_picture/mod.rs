use crate::account::Account;
use crate::station_picture_variant::{StationPictureVariant, StationPictureVariantFormat};
use crate::{run_transaction, Model};
use bytes::Bytes;
use mongodb::bson::doc;
use mongodb::{ClientSession, IndexModel};
use ril::{Paste, Rgba};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(StationPicture);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StationPicture {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,

  pub version: f64,

  pub src_filename: String,
  pub src_content_type: String,
  pub src_size: f64,
  pub src_size_bytes: f64,
  #[serde(rename = "src_sha256")]
  pub src_sha256: String,

  pub webp_sizes: Vec<f64>,
  pub png_sizes: Vec<f64>,

  pub created_at: DateTime,
  pub updated_at: DateTime,
}

impl Model for StationPicture {
  const CL_NAME: &'static str = "station_pictures";
  const UID_LEN: usize = 8;

  fn indexes() -> Vec<IndexModel> {
    vec![]
  }
}

impl StationPicture {
  pub const VERSION: f64 = 1.0;
  pub const PNG_SIZES: [f64; 2] = [192.0, 512.0];
  pub const WEBP_SIZES: [f64; 5] = [32.0, 64.0, 128.0, 256.0, 512.0];

  pub async fn delete_with_session(
    id: &str,
    session: &mut ClientSession,
  ) -> Result<bool, mongodb::error::Error> {
    let r1 = Self::delete_by_id_with_session(id, session).await?;
    let r2 = StationPictureVariant::cl()
      .delete_many_with_session(
        doc! { StationPictureVariant::KEY_PICTURE_ID: id },
        None,
        session,
      )
      .await?;

    let r = r1.deleted_count != 0 || r2.deleted_count != 0;

    Ok(r)
  }

  pub async fn create_variants(
    account_id: String,
    filename: String,
    content_type: String,
    data: Bytes,
  ) -> Result<(StationPicture, Vec<StationPictureVariant>), CreateStationPictureError> {
    tokio::task::spawn_blocking(
      move || -> Result<(StationPicture, Vec<StationPictureVariant>), CreateStationPictureError> {
        // use image::io::Reader;
        // use image::GenericImageView;
        // use std::io::Cursor;

        use ril::{Image, ImageFormat};

        let img = Image::<Rgba>::from_bytes_inferred(&data)?;

        // let img = match Reader::new(Cursor::new(data.as_ref())).with_guessed_format() {
        //   Err(e) => return Err(e.into()),
        //   Ok(reader) => match reader.decode() {
        //     Err(e) => return Err(e.into()),
        //     Ok(img) => img,
        //   },
        // };

        let (w, h) = img.dimensions();

        if w != h {
          return Err(CreateStationPictureError::ImageNotSquare);
        }

        if w < 512 {
          return Err(CreateStationPictureError::ImageTooSmallSize);
        }

        let now = DateTime::now();

        let doc = StationPicture {
          id: StationPicture::uid(),
          account_id: account_id.clone(),
          version: StationPicture::VERSION,
          src_filename: filename.clone(),
          src_content_type: content_type.clone(),
          src_size: w as f64,
          src_size_bytes: data.len() as f64,
          src_sha256: crypt::sha256(&data),
          png_sizes: StationPicture::PNG_SIZES.to_vec(),
          webp_sizes: StationPicture::WEBP_SIZES.to_vec(),
          created_at: now,
          updated_at: now,
        };

        let mut variants = vec![];

        let source = StationPictureVariant {
          id: StationPictureVariant::uid(),
          picture_id: doc.id.clone(),
          content_type: content_type.clone(),
          format: StationPictureVariantFormat::Source,
          size: w as f64,
          size_bytes: data.len() as f64,
          data,
          created_at: now,
          updated_at: now,
        };

        variants.push(source);

        for size in StationPicture::PNG_SIZES {
          // PNG images are used for Android apps icons so they get a white background
          let mut bg = Image::new(size as u32, size as u32, ril::Rgba::white());

          let mut img = img.clone();
          img.resize(size as u32, size as u32, ril::ResizeAlgorithm::Lanczos3);

          let cmd = Paste {
            position: (0, 0),
            image: &img,
            mask: None,
            overlay: Some(ril::OverlayMode::Merge),
          };

          bg.draw(&cmd);
          // let img = img.resize(
          //   size as u32,
          //   size as u32,
          //   image::imageops::FilterType::Lanczos3,
          // );

          // bg.paste(0, 0, &img);

          let img = bg;

          let mut buf = vec![];
          img.encode(ImageFormat::Png, &mut buf)?;

          // match img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png) {
          //   Ok(()) => {}
          //   Err(e) => return Err(e.into()),
          // };

          let variant = StationPictureVariant {
            id: StationPictureVariant::uid(),
            picture_id: doc.id.clone(),
            content_type: String::from("image/png"),
            format: StationPictureVariantFormat::Png,
            size,
            size_bytes: buf.len() as f64,
            data: Bytes::from(buf),
            created_at: now,
            updated_at: now,
          };

          variants.push(variant);
        }

        for size in StationPicture::WEBP_SIZES {
          let mut img = img.clone();
          img.resize(size as u32, size as u32, ril::ResizeAlgorithm::Lanczos3);

          // let img = img.resize(
          //   size as u32,
          //   size as u32,
          //   image::imageops::FilterType::Lanczos3,
          // );

          let mut buf = vec![];
          img.encode(ImageFormat::WebP, &mut buf)?;

          // let encoder = match Encoder::from_image(&img) {
          //   Err(s) => return Err(CreateStationPictureError::Webp(String::from(s))),
          //   Ok(encoder) => encoder,
          // };

          // quality 0-100
          // let webp = encoder.encode(90f32);

          // let data = Bytes::copy_from_slice(&webp);

          let variant = StationPictureVariant {
            id: StationPictureVariant::uid(),
            picture_id: doc.id.clone(),
            content_type: String::from("image/webp"),
            format: StationPictureVariantFormat::Webp,
            size,
            size_bytes: buf.len() as f64,
            data: Bytes::from(buf),
            created_at: now,
            updated_at: now,
          };

          variants.push(variant);
        }

        Ok((doc, variants))
      },
    )
    .await
    .unwrap()
  }

  pub async fn create(
    account_id: String,
    filename: String,
    content_type: String,
    data: Bytes,
  ) -> Result<(StationPicture, Vec<StationPictureVariant>), CreateStationPictureError> {
    let (doc, variants) =
      Self::create_variants(account_id.clone(), filename, content_type, data).await?;

    run_transaction!(session => {
      match tx_try!(Account::exists_with_session(&*account_id, &mut session).await) {
        false => return Err(CreateStationPictureError::AccountNotFound(account_id)),
        true => {},
      };

      tx_try!(StationPicture::insert_with_session(&doc, &mut session).await);
      tx_try!(StationPictureVariant::insert_many_with_session(&variants, &mut session).await);
    });

    Ok((doc, variants))
  }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateStationPictureError {
  #[error("mongo: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("Image is too large, max size is 2 MB")]
  ImageTooLargeBytes,
  #[error("Image must be square")]
  ImageNotSquare,
  #[error("Image must be of 512x512px or larger")]
  ImageTooSmallSize,
  #[error("Account with id {0} not found")]
  AccountNotFound(String),
  #[error("Image is not supported or invalid ({0})")]
  Ril(ril::Error),
}

impl From<ril::Error> for CreateStationPictureError {
  fn from(e: ril::Error) -> Self {
    Self::Ril(e)
  }
}
