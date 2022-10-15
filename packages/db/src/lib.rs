use mongodb::{Client, Database};
use once_cell::sync::Lazy;
use tokio::runtime::Handle;

pub mod account;
pub mod audio_chunk;
pub mod audio_file;
pub mod audio_upload_operation;
pub mod station;

static CLIENT: Lazy<Client> = Lazy::new(|| {
  tokio::task::block_in_place(|| {
    Handle::current().block_on(async {
      Client::with_uri_str("mongodb://localhost:27017/openstream-rs?replicaSet=rs1")
        .await
        .expect("mongodb connection")
    })
  })
});

pub fn client() -> Client {
  CLIENT.clone()
}

pub fn db() -> Database {
  client()
    .default_database()
    .expect("No database specified in mongodb connection string")
}

#[macro_export]
macro_rules! model {
  ($ty:ty) => {
    pub fn uid() -> String {
      uid::uid(UID_LEN)
    }

    pub fn cl() -> ::mongodb::Collection<$ty> {
      cl_as()
    }

    pub fn cl_as<T>() -> ::mongodb::Collection<T> {
      crate::db().collection(CL_NAME)
    }

    pub async fn insert(
      doc: impl ::std::borrow::Borrow<$ty>,
    ) -> Result<::mongodb::results::InsertOneResult, ::mongodb::error::Error> {
      cl().insert_one(doc, None).await
    }

    pub async fn replace(
      id: impl AsRef<str>,
      replacement: impl ::std::borrow::Borrow<$ty>,
    ) -> Result<::mongodb::results::UpdateResult, ::mongodb::error::Error> {
      cl()
        .replace_one(
          ::mongodb::bson::doc! {"_id": id.as_ref()},
          replacement,
          None,
        )
        .await
    }
  };
}
