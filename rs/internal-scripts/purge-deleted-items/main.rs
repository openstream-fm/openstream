#![allow(unreachable_code)]

use anyhow::Context;
use db::{
  access_token::AccessToken, account::Account, account_invitations::AccountInvitation,
  admin::Admin, audio_chunk::AudioChunk, audio_file::AudioFile,
  audio_upload_operation::AudioUploadOperation, email_verification_code::EmailVerificationCode,
  media_session::MediaSession, play_history_item::PlayHistoryItem, relay_session::RelaySession,
  sent_email::SentEmail, station::Station,
  station_files_pre_shuffle_checkpoint::StationFilesPreShuffleCheckpoint,
  station_picture::StationPicture, station_picture_variant::StationPictureVariant,
  token_user_recovery::TokenUserRecovery, user::User, user_account_relation::UserAccountRelation,
};
use log::*;
use mongodb::bson::doc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  shared_init().await?;
  purge().await?;
  Ok(())
}

async fn shared_init() -> Result<(), anyhow::Error> {
  logger::init();
  let _ = dotenv::dotenv();

  let client_options = mongodb::options::ClientOptions::parse(
    &std::env::var("OPENSTREAM_PURGE_MONGO_URL")
      .context("env.OPENSTREAM_PURGE_MONGO_URL is required")?,
  )
  .await
  .context("failed to parse mongodb connection string")?;

  info!("mongodb config hosts: {:?}", client_options.hosts);
  info!(
    "mongodb client compressors: {:?}",
    client_options.compressors
  );

  let client =
    mongodb::Client::with_options(client_options).context("failed to create mongodb client")?;

  if client.default_database().is_none() {
    anyhow::bail!("no database specified in config, under [mongodb] url");
  }

  info!("mongodb client created");

  db::init(
    client,
    Some(
      std::env::var("OPENSTREAM_PURGE_STORAGE_DB_NAME")
        .context("env.OPENSTREAM_PURGE_STORAGE_DB_NAME is required")?,
    ),
  );

  // info!("ensuring mongodb collections...");
  // db::ensure_collections()
  //   .await
  //   .context("error ensuring mongodb collections and indexes")?;

  // Ok(config)
  Ok(())
}

async fn purge() -> Result<(), mongodb::error::Error> {
  use db::stream_connection::lite::StreamConnectionLite;
  use db::stream_connection::StreamConnection;
  use db::Model;

  db::run_transaction!(session => {
    macro_rules! get_all {
      ($ty:ty) => {{
        let mut cursor = tx_try!(<$ty>::cl().find_with_session(None, None, &mut session).await);
        let mut docs = Vec::<$ty>::new();
        let mut index = HashMap::<String, $ty>::new();
        while let Some(doc) = tx_try!(cursor.next(&mut session).await.transpose()) {
          index.insert(doc.id.clone(), doc.clone());
          docs.push(doc);
        }
        info!("{} {}", docs.len(), stringify!($ty));
        (docs, index)
      }}
    }

    macro_rules! split {
      ($ident:ident) => {{
        let mut current_docs = vec![];
        let mut current_index = HashMap::new();
        let mut deleted_docs = vec![];
        let mut deleted_index = HashMap::new();
        for item in $ident.iter() {
          if item.deleted_at.is_none() {
            current_docs.push(item.clone());
            current_index.insert(item.id.clone(), item.clone());
          } else {
            deleted_docs.push(item.clone());
            deleted_index.insert(item.id.clone(), item.clone());
          }
        }

        info!(
          "{} current {} - {} deleted {}",
          current_docs.len(),
          stringify!($ident),
          deleted_docs.len(),
          stringify!($ident),
        );

        (current_docs, current_index, deleted_docs, deleted_index)
      }}
    }

    macro_rules! delete_ids {
      ($ty:ty, $docs:ident) => {{
        let ids = $docs.iter().map(|doc| doc.id.clone()).collect::<Vec<String>>();
        let filter = doc!{ "_id": { "$in": ids } };
        let r = tx_try!(<$ty>::cl().delete_many_with_session(filter, None, &mut session).await);
        info!(
          "{} {} deleted",
          r.deleted_count,
          stringify!($ty)
        );
      }}
    }

    info!("purge transaction started");

    let (admins, _admins_index) = get_all!(Admin);
    let (users, _users_index) = get_all!(User);
    let (accounts, _accounts_index) = get_all!(Account);
    let (stations, _stations_index) = get_all!(Station);
    let (station_pictures, _station_pictures_index) = get_all!(StationPicture);

    //let (user_account_relations, user_account_relations_index) = get_all!(UserAccountRelation);
    //let (access_tokens, access_tokens_index) = get_all!(AccessToken);
    //let (audio_files, audio_files_index) = get_all!(AudioFile);
    //let (audio_chunks, audio_chunks_index) = get_all!(AudioChunk);
    //let (account_invitations, account_invitations_index) = get_all!(AccountInvitation);
    //let (play_history_items, play_history_items_index) = get_all!(PlayHistoryItem);
    //let (stream_connections, stream_connections_index) = get_all!(StreamConnection);
    //let (stream_connections_lite, stream_connections_lite_index) = get_all!(StreamConnectionLite);
    //let (token_user_recoverys, token_user_recoverys_item) = get_all!(TokenUserRecovery);
    //let (email_verification_code, email_verification_code_index) = get_all!(EmailVerificationCode);
    //let (sent_email, sent_email_index) = get_all!(SentEmail);
    //let (pre_shuffly_checkpoints, pre_shuffle_checkpoints_index) = get_all!(StationFilesPreShuffleCheckpoint);
    //let (media_sessions, media_sessions_index) = get_all!(MediaSession);
    //let (relay_sessions, relay_sessions_index) = get_all!(RelaySession);

    info!("== GET stage ended ==");

    let (
      _current_admins,
      _current_admins_index,
      deleted_admins,
      _deleted_admins_index,
    ) = split!(admins);
    delete_ids!(Admin, deleted_admins);


    let (
      current_users,
      _current_users_index,
      deleted_users,
      _deleted_users_index,
    ) = split!(users);
    delete_ids!(User, deleted_users);

    let (
      current_accounts,
      current_accounts_index,
      deleted_accounts,
      _deleted_accounts_index,
    ) = split!(accounts);
    delete_ids!(Account, deleted_accounts);

    let (
      current_stations,
      _current_stations_index,
      deleted_stations,
      _deleted_stations_index,
    ) = split!(stations);

    info!("== SPLIT stage ended ==");

    let mut to_delete_stations = deleted_stations.clone();
    let mut current_station_ids = vec![];

    for item in current_stations {
      if !current_accounts_index.contains_key(&item.account_id) {
        info!("add to delete station => {} {} {}", item.id, item.name, item.account_id);
        to_delete_stations.push(item.clone());
      } else {
        current_station_ids.push(item.id.clone());
      }
    }

    delete_ids!(Station, to_delete_stations);


    let r = tx_try!(AccessToken::cl().delete_many_with_session(doc!{}, None, &mut session).await);
    info!("{} (all) access tokens deleted", r.deleted_count);

    let current_account_ids = current_accounts.iter().map(|item| item.id.clone()).collect::<Vec<String>>();
    let current_user_ids = current_users.iter().map(|item| item.id.clone()).collect::<Vec<String>>();

    // User Account Relationss
    {
      let filter = doc!{
        "$or": [
          { UserAccountRelation::KEY_ACCOUNT_ID: { "$nin": &current_account_ids } },
          { UserAccountRelation::KEY_USER_ID: { "$nin": &current_user_ids } }
        ]
      };

      let r = tx_try!(UserAccountRelation::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} user account relations deleted", r.deleted_count);
    }

    // Audio files
    {
      let filter = doc! {
        AudioFile::KEY_STATION_ID: { "$nin": &current_station_ids }
      };

      let r = tx_try!(AudioFile::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} audio files deleted", r.deleted_count);
    }

    // Audio files
    {
      let filter = doc! {
        AudioChunk::KEY_STATION_ID: { "$nin": &current_station_ids }
      };

      let r = tx_try!(AudioChunk::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} audio chunks deleted", r.deleted_count);
    }

    // Play History Items
    {
      let filter = doc! {
        PlayHistoryItem::KEY_STATION_ID: { "$nin": &current_station_ids }
      };

      let r = tx_try!(PlayHistoryItem::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} audio chunks deleted", r.deleted_count);
    }

    // Station pictures
    let mut current_station_pictures_ids = vec![];
    let mut station_pictures_to_delete = vec![];
    for item in station_pictures {
      if current_account_ids.contains(&item.account_id) {
        current_station_pictures_ids.push(item.id.clone());
      } else {
        station_pictures_to_delete.push(item.clone());
      }
    }

    delete_ids!(StationPicture, station_pictures_to_delete);

    // Station pictures variants
    {
      let filter = doc!{ StationPictureVariant::KEY_PICTURE_ID: { "$nin": &current_station_pictures_ids } };
      let r = tx_try!(StationPictureVariant::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} station picture variants deleted", r.deleted_count);
    }

    {
      let filter = doc!{ AccountInvitation::KEY_ACCOUNT_ID: { "$nin": &current_account_ids } };
      let r = tx_try!(AccountInvitation::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} account invitations deleted", r.deleted_count);
    }

    {
      let filter = doc!{ StreamConnection::KEY_STATION_ID: { "$nin": &current_station_ids } };
      let r = tx_try!(StreamConnection::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} stream connections deleted", r.deleted_count);
    }

    {
      let filter = doc!{ StreamConnectionLite::KEY_STATION_ID: { "$nin": &current_station_ids } };
      let r = tx_try!(StreamConnectionLite::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} stream connections lite deleted", r.deleted_count);
    }

    {
      let r = tx_try!(AudioUploadOperation::cl().delete_many_with_session(doc!{}, None, &mut session).await);
      info!("{} (all) upload operations deleted", r.deleted_count);
    }

    {
      let r = tx_try!(TokenUserRecovery::cl().delete_many_with_session(doc!{}, None, &mut session).await);
      info!("{} (all) token user recoveries deleted", r.deleted_count);
    }

    {
      let r = tx_try!(StationFilesPreShuffleCheckpoint::cl().delete_many_with_session(doc!{}, None, &mut session).await);
      info!("{} (all) shuffle checkpoints deleted", r.deleted_count);
    }

    {
      let r = tx_try!(EmailVerificationCode::cl().delete_many_with_session(doc!{}, None, &mut session).await);
      info!("{} (all) email verification codes deleted", r.deleted_count);
    }

    {
      let r = tx_try!(SentEmail::cl().delete_many_with_session(doc!{}, None, &mut session).await);
      info!("{} (all) sent emails deleted", r.deleted_count);
    }

    {
      let filter = doc! { MediaSession::KEY_STATION_ID: { "$nin": &current_station_ids } };
      let r = tx_try!(MediaSession::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} media sessions deleted", r.deleted_count);
    }

    {
      let filter = doc! { MediaSession::KEY_STATION_ID: { "$nin": &current_station_ids } };
      let r = tx_try!(RelaySession::cl().delete_many_with_session(filter, None, &mut session).await);
      info!("{} relay sessions deleted", r.deleted_count);
    }

    //session.abort_transaction().await?;
    //info!("transaction aborted");
  });

  Ok(())
}
