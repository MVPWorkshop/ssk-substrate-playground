use crate::types::ESupportedPallets;
use std::collections::HashSet;

pub fn remove_duplicate_pallets(pallets: &mut Vec<ESupportedPallets>) {
    let mut seen = HashSet::new();
    pallets.retain(|pallet| seen.insert(pallet.clone()));
}
