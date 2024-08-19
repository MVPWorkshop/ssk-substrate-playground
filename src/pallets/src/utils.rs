use super::types::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

pub fn pallets_config_to_model(config: PalletConfig) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("name".to_string(), config.name.to_string());
    map.insert("size".to_string(), config.metadata.size.to_string());
    map.insert("downloads".to_string(), "0".to_string());
    map.insert(
        "package_name".to_string(),
        config.dependencies.pallet.package,
    );
    map.insert("version".to_string(), config.dependencies.pallet.version);
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
                    // ... handle other categories
                    _ => "Other".to_string(),
                },
            );
            map
        })
        .collect()
}
