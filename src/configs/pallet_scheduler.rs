use super::super::pallet_index::pallet_index::SCHEDULER;
use super::super::types::*;
use chrono::Utc;
use std::fmt;

// The Scheduler pallet is dependent on the Treasury and Assets pallets, and they must be invoked together.

//TODO
//add pallet Preimage
//change  PalletSchedulerTraits::Preimages.to_string(), "()".to_string()
//with    PalletSchedulerTraits::Preimages.to_string(), "Preimage".to_string()

#[derive(Debug, Clone, Copy)]
pub enum PalletSchedulerTraits {
    RuntimeEvent,
    RuntimeOrigin,
    PalletsOrigin,
    RuntimeCall,
    MaximumWeight,
    ScheduleOrigin,
    OriginPrivilegeCmp,
    MaxScheduledPerBlock,
    WeightInfo,
    Preimages,
}

impl fmt::Display for PalletSchedulerTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletSchedulerTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletSchedulerTraits::RuntimeOrigin => write!(f, "RuntimeOrigin"),
            PalletSchedulerTraits::PalletsOrigin => write!(f, "PalletsOrigin"),
            PalletSchedulerTraits::RuntimeCall => write!(f, "RuntimeCall"),
            PalletSchedulerTraits::MaximumWeight => write!(f, "MaximumWeight"),
            PalletSchedulerTraits::ScheduleOrigin => write!(f, "ScheduleOrigin"),
            PalletSchedulerTraits::OriginPrivilegeCmp => write!(f, "OriginPrivilegeCmp"),
            PalletSchedulerTraits::MaxScheduledPerBlock => write!(f, "MaxScheduledPerBlock"),
            PalletSchedulerTraits::WeightInfo => write!(f, "WeightInfo"),
            PalletSchedulerTraits::Preimages => write!(f, "Preimages"),
        }
    }
}
pub struct PalletSchedulerConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletSchedulerConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "The Scheduler pallet allows scheduling the execution of specific calls at a designated block or within a specific time period. These scheduled calls can be named or anonymous, with the option to cancel them before execution."    ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME scheduler pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Runtime]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-scheduler".to_string(),
                version: None,
                alias: "pallet scheduler".to_string(),
                default_features: false,
                git_repo: Some("https://github.com/paritytech/polkadot-sdk.git".to_string()),
                tag: Some("polkadot-v1.14.0".to_string()),
                branch: None,
            },
            additional_pallets: None,
            additional_deps: None,
        };
        let runtime = PalletRuntimeConfig {
            construct_runtime: PalletConstructRuntimeConfig {
                index: Some(SCHEDULER),
                runtime: (
                    "Scheduler".to_string(),
                    "pallet_scheduler::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletSchedulerTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletSchedulerTraits::RuntimeOrigin.to_string(),
                    "RuntimeOrigin".to_string(),
                ),
                (
                    PalletSchedulerTraits::PalletsOrigin.to_string(),
                    "OriginCaller".to_string(),
                ),
                (
                    PalletSchedulerTraits::RuntimeCall.to_string(),
                    "RuntimeCall".to_string(),
                ),
                (
                    PalletSchedulerTraits::MaximumWeight.to_string(),
                    "MaximumSchedulerWeight".to_string(),
                ),
                (
                    PalletSchedulerTraits::ScheduleOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletSchedulerTraits::OriginPrivilegeCmp.to_string(),
                    "EqualPrivilegeOnly".to_string(),
                ),
                (
                    PalletSchedulerTraits::MaxScheduledPerBlock.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletSchedulerTraits::WeightInfo.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletSchedulerTraits::Preimages.to_string(),
                    "()".to_string(), // type Preimages = Preimage; when impl pallet Preimage
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![String::from(
                "use frame_support::traits::EqualPrivilegeOnly;",
            )]),
            runtime_api_code: None,
        };

        PalletSchedulerConfig {
            name: "Pallet Scheduler".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}
fn get_additional_implementation_code() -> String {
    "
parameter_types! {
        pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
        }
"
    .to_string()
}
