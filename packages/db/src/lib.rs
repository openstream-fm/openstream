use mongodb::{Client, Database};
use tokio::runtime::Handle;
use once_cell::sync::Lazy;

pub mod account;
pub mod station;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    tokio::task::block_in_place(|| {
        Handle::current().block_on(async {
            Client::with_uri_str("mongodb://localhost:27017/openstream-rs?replicaSet=rs1").await.expect("mongodb connection")
        })
    })
});

pub fn client() -> Client {
    CLIENT.clone()
}

pub fn db() -> Database {
    client().default_database().expect("No database specified in mongodb connection string")
}

#[macro_export]
macro_rules! model {
    ($ty:ty) => {
        
        pub fn uid() -> String {
            uid::uid(UID_LEN)
        }

        pub fn cl() -> mongodb::Collection<$ty> {
            cl_as()
        }

        pub fn cl_as<T>() -> mongodb::Collection<T> {
            crate::db().collection(CL_NAME)
        }
    }
}