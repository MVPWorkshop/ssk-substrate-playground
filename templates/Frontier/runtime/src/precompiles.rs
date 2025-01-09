use core::marker::PhantomData;
use pallet_evm::{
    IsPrecompileResult, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use sp_core::H160;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};

pub const ADDR_EC_RECOVER: [u8; 20] = address_of(0x01);
pub const ADDR_SHA256: [u8; 20] = address_of(0x02);
pub const ADDR_RIPEMD160: [u8; 20] = address_of(0x03);
pub const ADDR_IDENTITY: [u8; 20] = address_of(0x04);
pub const ADDR_MODEXP: [u8; 20] = address_of(0x05);
pub const ADDR_BN128_ADD: [u8; 20] = address_of(0x06);
pub const ADDR_BN128_MUL: [u8; 20] = address_of(0x07);
pub const ADDR_BN128_PAIRING: [u8; 20] = address_of(0x08);
pub const ADDR_BLAKE2F: [u8; 20] = address_of(0x09);
// [0x400, 0x800) for stable precompiles.
pub const ADDR_STATE_STORAGE: [u8; 20] = address_of(0x400);
pub const ADDR_DISPATCH: [u8; 20] = address_of(0x401);
// [0x800..) for the experimental precompiles.
pub const ADDR_EXPERIMENTAL: [u8; 20] = address_of(0x800);

pub struct FrontierPrecompiles<R>(PhantomData<R>);

impl<R> FrontierPrecompiles<R>
where
    R: pallet_evm::Config,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn used_addresses() -> [H160; 7] {
        [
            hash(1),
            hash(2),
            hash(3),
            hash(4),
            hash(5),
            hash(1024),
            hash(1025),
        ]
    }

    pub fn set() -> [[u8; 20]; 12] {
        [
            ADDR_EC_RECOVER,
            ADDR_SHA256,
            ADDR_RIPEMD160,
            ADDR_IDENTITY,
            ADDR_MODEXP,
            ADDR_BN128_ADD,
            ADDR_BN128_MUL,
            ADDR_BN128_PAIRING,
            ADDR_BLAKE2F,
            ADDR_STATE_STORAGE,
            ADDR_DISPATCH,
            ADDR_EXPERIMENTAL,
        ]
    }
}

pub const fn address_of(v: u64) -> [u8; 20] {
    [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        ((v >> 56) & 0xff) as u8,
        ((v >> 48) & 0xff) as u8,
        ((v >> 40) & 0xff) as u8,
        ((v >> 32) & 0xff) as u8,
        ((v >> 24) & 0xff) as u8,
        ((v >> 16) & 0xff) as u8,
        ((v >> 8) & 0xff) as u8,
        (v & 0xff) as u8,
    ]
}

impl<R> PrecompileSet for FrontierPrecompiles<R>
where
    R: pallet_evm::Config,
{
    fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
        match handle.code_address() {
            // Ethereum precompiles :
            a if a == hash(1) => Some(ECRecover::execute(handle)),
            a if a == hash(2) => Some(Sha256::execute(handle)),
            a if a == hash(3) => Some(Ripemd160::execute(handle)),
            a if a == hash(4) => Some(Identity::execute(handle)),
            a if a == hash(5) => Some(Modexp::execute(handle)),
            // Non-Frontier specific nor Ethereum precompiles :
            a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
            a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
            _ => None,
        }
    }

    fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
        IsPrecompileResult::Answer {
            is_precompile: Self::used_addresses().contains(&address),
            extra_cost: 0,
        }
    }
}

fn hash(a: u64) -> H160 {
    H160::from_low_u64_be(a)
}
