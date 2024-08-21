use actix_web::{web, App, HttpServer};
use pallets_configs::configs::pallet_utility::PalletUtilityConfig;
use pallets_configs::configs::pallet_aura::PalletAuraConfig;
use pallets_configs::configs::pallet_timestamp::PalletTimestampConfig;
use pallets_configs::types::PalletConfig;
use pallets_configs::utils::pallets_config_to_model;
use std::process;

async fn start_server() {
    HttpServer::new(|| {
        App::new()
        // ... setup routes
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .expect("REASON")
}

fn main() {
    // let server = start_server();
    // let _ = server.map_err(|e| {
    //     eprintln!("Server error: {}", e);
    //     process::exit(1);
    // });
    let pallet_utility_config = PalletUtilityConfig::new();
    println!("Pallet Utility Config: {:?}", pallet_utility_config);

    let pallet_timestamp_config = PalletTimestampConfig::new();
    println!("Pallet Timestamp Config: {:?}", pallet_timestamp_config);

    let utility_model = pallets_config_to_model(PalletConfig {
        name: pallet_utility_config.name,
        metadata: pallet_utility_config.metadata,
        runtime: pallet_utility_config.runtime,
        dependencies: pallet_utility_config.dependencies,
    });

    println!("Pallet Utility Model: {:?}", utility_model);

}
