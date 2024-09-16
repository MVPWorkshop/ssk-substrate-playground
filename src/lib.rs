pub mod code_generator;
pub mod types;
pub mod util;
pub mod configs {
    pub mod pallet_aura;
    pub mod pallet_balances;
    pub mod pallet_grandpa;
    pub mod pallet_identity;
    pub mod pallet_sudo;
    pub mod pallet_timestamp;
    pub mod pallet_transaction_payment;
    pub mod pallet_utility;
}

pub mod utils {
    pub mod file_manager;
    pub mod manifest;

    pub mod runtime;
}