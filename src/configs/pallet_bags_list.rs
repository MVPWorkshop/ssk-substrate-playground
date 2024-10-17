use super::super::pallet_index::BAGS_LIST;
use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletBagsListTraits {
    RuntimeEvent,
    ScoreProvider,
    BagThresholds,
    Score,
    WeightInfo,
}

impl fmt::Display for PalletBagsListTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletBagsListTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletBagsListTraits::ScoreProvider => write!(f, "ScoreProvider"),
            PalletBagsListTraits::BagThresholds => write!(f, "BagThresholds"),
            PalletBagsListTraits::Score => write!(f, "Score"),
            PalletBagsListTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalletBagsListConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletBagsListConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PalletBagsListConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "An on-chain implementation of a semi-sorted linked list, with permission-less sorting and update operations.",
        ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME bags list pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Governance]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-bags-list".to_string(),
                version: None,
                alias: "pallet bags list".to_string(),
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
                index: Some(BAGS_LIST),
                runtime: (
                    "VoterList".to_string(),
                    "pallet_bags_list<Instance1>".to_string(),
                ),
            },
            pallet_traits: vec![].into_iter().collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: None,
            runtime_api_code: None,
        };

        PalletBagsListConfig {
            name: "Pallet bags list".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
/// Upper thresholds delimiting the bag list.
pub const THRESHOLDS: [u64; 200] = [
    100_000_000_000_000,
    106_282_535_907_434,
    112_959_774_389_150,
    120_056_512_776_105,
    127_599_106_300_477,
    135_615_565_971_369,
    144_135_662_599_590,
    153_191_037_357_827,
    162_815_319_286_803,
    173_044_250_183_800,
    183_915_817_337_347,
    195_470_394_601_017,
    207_750_892_330_229,
    220_802_916_738_890,
    234_674_939_267_673,
    249_418_476_592_914,
    265_088_281_944_639,
    281_742_548_444_211,
    299_443_125_216_738,
    318_255_747_080_822,
    338_250_278_668_647,
    359_500_973_883_001,
    382_086_751_654_776,
    406_091_489_025_036,
    431_604_332_640_068,
    458_720_029_816_222,
    487_539_280_404_019,
    518_169_110_758_247,
    550_723_271_202_866,
    585_322_658_466_782,
    622_095_764_659_305,
    661_179_154_452_653,
    702_717_972_243_610,
    746_866_481_177_808,
    793_788_636_038_393,
    843_658_692_126_636,
    896_661_852_395_681,
    952_994_955_240_703,
    1_012_867_205_499_736,
    1_076_500_951_379_881,
    1_144_132_510_194_192,
    1_216_013_045_975_769,
    1_292_409_502_228_280,
    1_373_605_593_276_862,
    1_459_902_857_901_004,
    1_551_621_779_162_291,
    1_649_102_974_585_730,
    1_752_708_461_114_642,
    1_862_822_999_536_805,
    1_979_855_523_374_646,
    2_104_240_657_545_975,
    2_236_440_332_435_128,
    2_376_945_499_368_703,
    2_526_277_953_866_680,
    2_684_992_273_439_945,
    2_853_677_877_130_641,
    3_032_961_214_443_876,
    3_223_508_091_799_862,
    3_426_026_145_146_232,
    3_641_267_467_913_124,
    3_870_031_404_070_482,
    4_113_167_516_660_186,
    4_371_578_742_827_277,
    4_646_224_747_067_156,
    4_938_125_485_141_739,
    5_248_364_991_899_922,
    5_578_095_407_069_235,
    5_928_541_253_969_291,
    6_301_003_987_036_955,
    6_696_866_825_051_405,
    7_117_599_888_008_300,
    7_564_765_656_719_910,
    8_040_024_775_416_580,
    8_545_142_218_898_723,
    9_081_993_847_142_344,
    9_652_573_371_700_016,
    10_258_999_759_768_490,
    10_903_525_103_419_522,
    11_588_542_983_217_942,
    12_316_597_357_287_042,
    13_090_392_008_832_678,
    13_912_800_587_211_472,
    14_786_877_279_832_732,
    15_715_868_154_526_436,
    16_703_223_214_499_558,
    17_752_609_210_649_358,
    18_867_923_258_814_856,
    20_053_307_312_537_008,
    21_313_163_545_075_252,
    22_652_170_697_804_756,
    24_075_301_455_707_600,
    25_587_840_914_485_432,
    27_195_406_207_875_088,
    28_903_967_368_057_400,
    30_719_869_496_628_636,
    32_649_856_328_471_220,
    34_701_095_276_033_064,
    36_881_204_047_022_752,
    39_198_278_934_370_992,
    41_660_924_883_519_016,
    44_278_287_448_695_240,
    47_060_086_756_856_400,
    50_016_653_605_425_536,
    53_158_967_827_883_320,
    56_498_699_069_691_424,
    60_048_250_125_977_912,
    63_820_803_001_928_304,
    67_830_367_866_937_216,
    72_091_835_084_322_176,
    76_621_030_509_822_880,
    81_434_774_264_248_528,
    86_550_943_198_537_824,
    91_988_537_283_208_848,
    97_767_750_168_749_840,
    103_910_044_178_992_000,
    110_438_230_015_967_792,
    117_376_551_472_255_616,
    124_750_775_465_407_920,
    132_588_287_728_824_640,
    140_918_194_514_440_064,
    149_771_430_684_917_568,
    159_180_874_596_775_264,
    169_181_470_201_085_280,
    179_810_356_815_193_344,
    191_107_007_047_393_216,
    203_113_373_386_768_288,
    215_874_044_002_592_672,
    229_436_408_331_885_600,
    243_850_833_070_063_392,
    259_170_849_218_267_264,
    275_453_350_882_006_752,
    292_758_806_559_399_232,
    311_151_483_703_668_992,
    330_699_687_393_865_920,
    351_476_014_000_157_824,
    373_557_620_785_735_808,
    397_026_512_446_556_096,
    421_969_845_653_044_224,
    448_480_252_724_740_928,
    476_656_185_639_923_904,
    506_602_281_657_757_760,
    538_429_751_910_786_752,
    572_256_794_410_890_176,
    608_209_033_002_485_632,
    646_419_983_893_124_352,
    687_031_551_494_039_552,
    730_194_555_412_054_016,
    776_069_290_549_944_960,
    824_826_122_395_314_176,
    876_646_119_708_695_936,
    931_721_726_960_522_368,
    990_257_479_014_182_144,
    1_052_470_760_709_299_712,
    1_118_592_614_166_106_112,
    1_188_868_596_808_997_376,
    1_263_559_693_295_730_432,
    1_342_943_284_738_898_688,
    1_427_314_178_819_094_784,
    1_516_985_704_615_302_400,
    1_612_290_876_218_400_768,
    1_713_583_629_449_105_408,
    1_821_240_136_273_157_632,
    1_935_660_201_795_120_128,
    2_057_268_749_018_809_600,
    2_186_517_396_888_336_384,
    2_323_886_137_470_138_880,
    2_469_885_118_504_583_168,
    2_625_056_537_947_004_416,
    2_789_976_657_533_970_944,
    2_965_257_942_852_572_160,
    3_151_551_337_860_326_400,
    3_349_548_682_302_620_672,
    3_559_985_281_005_267_968,
    3_783_642_634_583_792_128,
    4_021_351_341_710_503_936,
    4_273_994_183_717_548_544,
    4_542_509_402_991_247_872,
    4_827_894_187_332_742_144,
    5_131_208_373_224_844_288,
    5_453_578_381_757_959_168,
    5_796_201_401_831_965_696,
    6_160_349_836_169_256_960,
    6_547_376_026_650_146_816,
    6_958_717_276_519_173_120,
    7_395_901_188_113_309_696,
    7_860_551_335_934_872_576,
    8_354_393_296_137_270_272,
    8_879_261_054_815_360_000,
    9_437_103_818_898_946_048,
    10_029_993_254_943_105_024,
    10_660_131_182_698_121_216,
    11_329_857_752_030_707_712,
    12_041_660_133_563_240_448,
    12_798_181_755_305_525_248,
    13_602_232_119_581_272_064,
    14_456_797_236_706_498_560,
    15_365_050_714_167_523_328,
    16_330_365_542_480_556_032,
    17_356_326_621_502_140_416,
    18_446_744_073_709_551_615,
];
parameter_types! {
    pub const BagThresholds: &'static [u64] = &THRESHOLDS;
}
type VoterBagsListInstance = pallet_bags_list::Instance1;

impl pallet_bags_list::Config<VoterBagsListInstance> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    /// The voter bags-list is loosely kept up to date, and the real source of truth for the score
    /// of each node is the staking pallet.
    type ScoreProvider = Staking;
    type BagThresholds = BagThresholds;
    type Score = VoteWeight;
    type WeightInfo = pallet_bags_list::weights::SubstrateWeight<Runtime>;
}
"
    .to_string()
}
