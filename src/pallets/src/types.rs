use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum PalletModuleParts {
    Module,
    Call,
    Storage,
    Event,
    Origin,
    Config,
    Inherent,
    ValidateUnsigned,
}

#[derive(Debug, Clone)]
pub struct PalletConstructRuntimeConfig {
    pub modules: Vec<PalletModuleParts>,
    pub generic: Option<HashMap<PalletModuleParts, bool>>,
}

#[derive(Debug, Clone)]
pub struct PalletTraitsConfig {
    custom_name: Option<String>,
    type_: String,
    value: String,
    is_not_const: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct PalletGenesisConfig {
    config_struct_name: String,
    struct_fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PalletRuntimeConfig {
    pub construct_runtime: PalletConstructRuntimeConfig,
    pub pallet_traits: HashMap<String, String>,
    pub genesis_config: Option<PalletGenesisConfig>,
    pub additional_chain_spec_code: Option<HashMap<String, Vec<String>>>,
    pub additional_runtime_lib_code: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct CargoSimpleDependency {
    package: String,
    version: String,
}

#[derive(Debug, Clone)]
pub struct CargoComplexDependency {
    pub package: String,
    pub version: String,
    pub alias: String,
    pub default_features: Option<Vec<String>>,
    pub git_repo: Option<String>,
    pub tag: Option<String>,
    pub branch: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PalletDependencyConfig {
    pub pallet: CargoComplexDependency,
    pub additional_pallets: Option<Vec<HashMap<String, bool>>>,
    pub additional_deps: Option<Vec<CargoSimpleDependency>>,
}

#[derive(Debug, Clone)]
pub enum SubstrateVersion {
    One,
    Two,
}

#[derive(Debug, Clone)]
pub enum PalletCategories {
    Accounts,
    Assets,
    Consensus,
    Governance,
    Identity,
    Runtime,
    SmartContracts,
    Other,
}

#[derive(Debug, Clone)]
pub enum CommonAuthors {
    ParityTechnologies,
    IndividualDevelopers,
    SubstrateDevHub,
}

#[derive(Debug, Clone)]
pub struct PalletMetadata {
    pub description: String,
    pub short_description: String,
    pub compatibility: SubstrateVersion,
    pub license: Option<String>,
    pub authors: Vec<CommonAuthors>,
    pub categories: Option<Vec<PalletCategories>>,
    pub size: usize,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PalletConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

#[derive(Debug, Clone)]
pub enum ESupportedPallets {
    PalletContract,
    PalletBalance,
    PalletNicks,
    PalletAura,
    PalletGrandpa,
    PalletRandomnessCollectiveFlip,
    PalletSudo,
    PalletTimestamp,
    PalletTransactionPayment,
    PalletRecovery,
    PalletVesting,
    PalletAssets,
    PalletSession,
    PalletBabe,
    PalletAuthorityDiscovery,
    PalletAuthorship,
    PalletFinalityTracker,
    PalletOffences,
    PalletImOnline,
    PalletAtomicSwap,
    PalletStaking,
    PalletMultisig,
    PalletUtility,
    PalletIndices,
    PalletCollective,
    PalletElectionsPhragmen,
    PalletElections,
    PalletMembership,
    PalletTreasury,
    PalletIdentity,
    PalletScheduler,
    PalletDemocracy,
    PalletSociety,
    PalletScoredPool,
    PalletEvm,
    PalletProxy,
    PalletDid,
    PalletRegistrar,
    PalletProductRegistry,
    PalletProductTracking,
    PalletRbac,
    PalletValidatorSet,
}
