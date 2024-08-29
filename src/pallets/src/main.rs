use actix_web::{web, App, HttpServer};
// use pallets_configs::configs::pallet_aura::PalletAuraConfig;
// use pallets_configs::configs::pallet_balances::PalletBalancesConfig;
// use pallets_configs::configs::pallet_timestamp::PalletTimestampConfig;
use pallets_configs::configs::pallet_utility::PalletUtilityConfig;
use pallets_configs::types::PalletConfig;
use pallets_configs::util::pallets_config_to_model;
use pallets_configs::utils::manifest::ManifestPalletConfig;
use pallets_configs::utils::manifest::SubstrateManifestUtil;
use pallets_configs::utils::runtime::{SubstrateRuntimeUtil, GeneratedRuntime};
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

use std::fs::File;
use std::io::{self, Read};

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    // Open the file in read-only mode
    let mut file = File::open(file_path)?;

    // Create a String to hold the file content
    let mut content = String::new();

    // Read the file content into the String
    file.read_to_string(&mut content)?;

    // Return the file content as a Result<String>
    Ok(content)
}
fn main() {
    // let server = start_server();
    // let _ = server.map_err(|e| {
    //     eprintln!("Server error: {}", e);
    //     process::exit(1);
    // });
    let pallet_utility_config = PalletUtilityConfig::new();
    // println!("Pallet Utility Config: {:?}", pallet_utility_config);

    // let pallet_timestamp_config = PalletTimestampConfig::new();
    // println!("Pallet Timestamp Config: {:?}", pallet_timestamp_config);
    //
    // let pallet_balances_config = PalletBalancesConfig::new();
    // println!("Pallet Balances Config: {:?}", pallet_balances_config);
    //
    // let utility_model = pallets_config_to_model(PalletConfig {
    //     name: pallet_utility_config.name,
    //     metadata: pallet_utility_config.metadata,
    //     runtime: pallet_utility_config.runtime,
    //     dependencies: pallet_utility_config.dependencies,
    // });
    //
    // println!("Pallet Utility Model: {:?}", utility_model);

    //     let pallet_config = ManifestPalletConfig {
    //         name: "utility_pallet".to_string(),
    //         dependencies: pallet_utility_config.dependencies,
    //     };
    //
    //     let mut runtime_manifest = r#"
    // [dependencies]
    // serde = "1.0"
    //
    // [features]
    // default = ["std"]
    // std = [""codec/std","]
    // "#
    //     .to_string();
    //
    //     // Specify the file path
    //     let file_path = "../../templates/solochain/basic/runtime/cargo.toml";
    //
    //     // Call the function to read the file
    //     match read_file_to_string(file_path) {
    //         Ok(content) => {
    //             println!("File content:\n{}", content);
    //             let mut util = SubstrateManifestUtil::new(pallet_config, content);
    //             let updated_manifest = util.generate_code();
    //             println!("{}", updated_manifest);
    //         }
    //         Err(e) => {
    //             println!("Failed to read the file: {}", e);
    //         }
    //     }

    // Specify the file path
    let runtime_file_path = "../../templates/solochain/basic/runtime/src/lib.rs";

    let chainspec_file_path = "../../templates/solochain/basic/node/src/chain_spec.rs";

    // Call the function to read the file
    let runtime_string = read_file_to_string(runtime_file_path).unwrap();

    let chainspec_string = read_file_to_string(chainspec_file_path).unwrap();

    // println!("File content:\n{}", runtime_string);
    // println!("File content:\n{}", chainspec_string);

    let config = PalletConfig {
        name: pallet_utility_config.name,
        metadata: pallet_utility_config.metadata,
        runtime: pallet_utility_config.runtime,
        dependencies: pallet_utility_config.dependencies,
    };

    let mut util = SubstrateRuntimeUtil::new(config, runtime_string, chainspec_string);
    // println!("{:#?}", util);
    let updated_runtime = util.generate_code();
    println!("{}", updated_runtime.updated_runtime_code);
}
