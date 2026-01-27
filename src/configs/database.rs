use mongodb::{Client, Collection, Database, options::ClientOptions};

use crate::{
    consts::{get_mongodb_db_name, get_mongodb_uri},
    structs::school::School,
};

/// Get MongoDB client.
pub async fn get_client() -> Client {
    let client_options: ClientOptions =
        ClientOptions::parse(get_mongodb_uri()).await.unwrap();

    Client::with_options(client_options).unwrap()
}

/// Get MongoDB database.
pub async fn get_database() -> Database {
    let client: Client = get_client().await;

    client.database(&get_mongodb_db_name())
}

/// Get MongoDB Schools collection.
pub async fn get_schools() -> Collection<School> {
    let db: Database = get_database().await;

    db.collection("schools")
}
