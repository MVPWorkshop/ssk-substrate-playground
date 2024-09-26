use mongodb::error::Error;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a struct to represent the document
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub name: String,
    pub description: String,
}

// Function to establish a connection to the MongoDB server
pub async fn connect_to_db(
    uri: &str,
    db_name: &str,
    collection_name: &str,
) -> Result<Collection<Record>, Error> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("Templates".to_string());

    // Initialize the client with the options
    let client = Client::with_options(client_options)?;

    // Get a handle to the database and collection
    let database = client.database(db_name);
    let collection = database.collection::<Record>(collection_name);

    Ok(collection)
}

// Function to insert a document into the collection
pub async fn insert_record(
    collection: &Collection<HashMap<String, String>>,
    record: HashMap<String, String>,
) -> Result<(), Error> {
    let result = collection.insert_one(record, None).await?;
    println!("Inserted document with id: {:?}", result.inserted_id);
    Ok(())
}

// Function to fetch a document by name
pub async fn fetch_record(
    collection: &Collection<Record>,
    name: &str,
) -> Result<Option<Record>, Error> {
    let filter = doc! { "name": name };
    let result = collection.find_one(filter, None).await?;
    Ok(result)
}
