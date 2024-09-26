use super::types::*;
use crate::code_generator::get_pallet_configs;
use crate::database::insert_record;
use crate::types::ESupportedPallets;
use mongodb::Collection;
use std::collections::HashMap;
use std::sync::Arc;
use strum::IntoEnumIterator;
use uuid::Uuid;

/// Inserts pallet data into the MongoDB database.
///
/// This function retrieves pallet configurations for supported pallets, transforms them
/// into a format compatible with MongoDB, and inserts them into the specified collection.
pub async fn insert_pallet_data_to_db() {
    // Retrieve the list of supported pallets.
    let pallets: Vec<_> = ESupportedPallets::iter().collect();

    // Get the configuration for each pallet.
    let pallet_configs = get_pallet_configs(pallets);

    // MongoDB connection parameters
    let uri: &str = "";
    let db_name: &str = "ParachainRuntimeBuilder";
    let collection_name: &str = "PalletInformation";

    // Connect to MongoDB using the URI
    let client = mongodb::Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    // Access the specified database and collection.
    let database = client.database(db_name);
    let collection: Collection<HashMap<String, String>> = database.collection(collection_name);
    let collection = Arc::new(collection);

    // Iterate over each pallet configuration and insert it into the database.
    for pallet_config in pallet_configs {
        // Convert pallet configuration into a HashMap model for MongoDB insertion.
        let config = pallet_config_to_model(pallet_config);

        // Attempt to insert the record into the MongoDB collection.
        match insert_record(&collection, config).await {
            Ok(_) => println!("Successfully inserted record."),
            Err(e) => println!("Error inserting record: {}", e),
        }
    }
}

/// Converts a `PalletConfig` into a HashMap<String, String> for database insertion.
///
/// This function takes a pallet configuration and converts it into a `HashMap` where each
/// key corresponds to a particular attribute of the pallet (e.g., name, size, description).
///
/// # Arguments
///
/// * `config` - A `PalletConfig` struct containing metadata and configuration information.
///
/// # Returns
///
/// A `HashMap<String, String>` where each key represents a field for MongoDB.
pub fn pallet_config_to_model(config: PalletConfig) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Insert various pallet attributes into the map.
    map.insert("name".to_string(), config.name.to_string());
    map.insert("size".to_string(), config.metadata.size.to_string());
    map.insert("downloads".to_string(), "0".to_string()); // Default value for downloads.

    // Insert the package name from dependencies.
    map.insert(
        "package_name".to_string(),
        config.dependencies.pallet.package,
    );

    // Insert the license (if available) or use default if none is provided.
    map.insert(
        "license".to_string(),
        config.metadata.license.unwrap_or_default(),
    );

    // Insert the last update date and the descriptions.
    map.insert(
        "package_last_update".to_string(),
        config.metadata.updated.to_string(),
    );
    map.insert("description".to_string(), config.metadata.description);
    map.insert(
        "short_description".to_string(),
        config.metadata.short_description,
    );

    // Convert the compatibility version to a string and insert.
    map.insert(
        "compatibility".to_string(),
        match config.metadata.compatibility {
            SubstrateVersion::One => "One".to_string(),
            SubstrateVersion::Two => "Two".to_string(),
        },
    );
    map
}

/// Converts the pallet categories from a `PalletConfig` into a vector of HashMaps.
///
/// Each category of the pallet will be converted into its own `HashMap` with fields such as
/// category name and a unique ID.
///
/// # Arguments
///
/// * `config` - A `PalletConfig` struct containing metadata and configuration information.
///
/// # Returns
///
/// A vector of `HashMap<String, String>`, each representing a category associated with the pallet.
pub fn pallets_config_to_categories_model(config: PalletConfig) -> Vec<HashMap<String, String>> {
    // Iterate through the categories of the pallet (if available) and convert each one into a map.
    config
        .metadata
        .categories
        .unwrap_or_default()
        .iter()
        .map(|category| {
            let mut map = HashMap::new();

            // Generate a unique ID for each category.
            map.insert("id".to_string(), Uuid::new_v4().to_string());

            // Insert the pallet name and the category name.
            map.insert("pallet_name".to_string(), config.name.clone());
            map.insert(
                "category".to_string(),
                match category {
                    PalletCategories::Accounts => "Accounts".to_string(),
                    PalletCategories::Assets => "Assets".to_string(),
                    PalletCategories::Consensus => "Consensus".to_string(),
                    PalletCategories::Governance => "Governance".to_string(),
                    PalletCategories::Identity => "Identity".to_string(),
                    PalletCategories::Runtime => "Runtime".to_string(),
                    PalletCategories::SmartContracts => "SmartContracts".to_string(),
                    _ => "Other".to_string(),
                },
            );
            map
        })
        .collect()
}
