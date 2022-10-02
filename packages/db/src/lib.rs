use mongodb::{Client, Database};
use lazy_static::lazy_static;

pub mod account;

lazy_static! {
    static ref CLIENT: Client = {
        todo!()
    };
}

pub fn client() -> Client {
    CLIENT.clone()
}

pub fn db() -> Database {
    client().default_database().expect("No database specified in mongodb connection string")
}