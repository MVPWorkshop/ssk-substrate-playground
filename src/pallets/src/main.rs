use actix_web::{web, App, HttpServer};
use pallets_configs::code_generator::generate_project;
use pallets_configs::configs::pallet_aura::PalletAuraConfig;
use pallets_configs::configs::pallet_balances::PalletBalancesConfig;
use pallets_configs::configs::pallet_timestamp::PalletTimestampConfig;
use pallets_configs::configs::pallet_utility::PalletUtilityConfig;
use pallets_configs::types::PalletConfig;
use pallets_configs::util::pallets_config_to_model;
use pallets_configs::utils::file_manager::{
    copy_dir_recursive, create_new_folder, read_file_to_string, replace_file_content,
};
use pallets_configs::utils::manifest::ManifestPalletConfig;
use pallets_configs::utils::manifest::SubstrateManifestUtil;
use pallets_configs::utils::runtime::{GeneratedRuntime, SubstrateRuntimeUtil};
use std::path::Path;
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

    generate_project();
}
