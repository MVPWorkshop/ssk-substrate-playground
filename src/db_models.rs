use super::types::*;
use crate::code_generator::get_pallet_configs;
use crate::database::insert_record;
use crate::types::ESupportedPallets;
use mongodb::Collection;
use std::collections::HashMap;
use std::sync::Arc;
use strum::IntoEnumIterator;
use uuid::Uuid;

pub async fn insert_pallet_data_to_db() {
    let pallets: Vec<_> = ESupportedPallets::iter().collect();

    let pallet_configs = get_pallet_configs(pallets);

    // MongoDB connection parameters
    let uri: &str = "mongodb+srv://pankajchaudhary:1234567890@dev.wn6dh.mongodb.net/retryWrites=true&w=majority&appName=dev";
    let db_name: &str = "ParachainRuntimeBuilder";
    let collection_name: &str = "PalletInformation";

    let client = mongodb::Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");
    let database = client.database(db_name);
    let collection: Collection<HashMap<String, String>> = database.collection(collection_name);
    let collection = Arc::new(collection);

    for pallet_config in pallet_configs {
        let config = pallet_config_to_model(pallet_config);

        match insert_record(&collection, config).await {
            Ok(_) => println!("Successfully inserted record."),
            Err(e) => println!("Error inserting record: {}", e),
        }
    }
}

pub fn pallet_config_to_model(config: PalletConfig) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("name".to_string(), config.name.to_string());
    map.insert("size".to_string(), config.metadata.size.to_string());
    map.insert("downloads".to_string(), "0".to_string());
    map.insert(
        "package_name".to_string(),
        config.dependencies.pallet.package,
    );
    map.insert(
        "license".to_string(),
        config.metadata.license.unwrap_or_default(),
    );
    map.insert(
        "package_last_update".to_string(),
        config.metadata.updated.to_string(),
    );
    map.insert("description".to_string(), config.metadata.description);
    map.insert(
        "short_description".to_string(),
        config.metadata.short_description,
    );
    map.insert(
        "compatibility".to_string(),
        match config.metadata.compatibility {
            SubstrateVersion::One => "One".to_string(),
            SubstrateVersion::Two => "Two".to_string(),
        },
    );
    map
}

pub fn pallets_config_to_categories_model(config: PalletConfig) -> Vec<HashMap<String, String>> {
    config
        .metadata
        .categories
        .unwrap_or_default()
        .iter()
        .map(|category| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Uuid::new_v4().to_string());
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
