// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Primitives of parachains module.

#![cfg_attr(not(feature = "std"), no_std)]

use bp_polkadot_core::{
	parachains::{ParaHash, ParaId},
	BlockNumber as RelayBlockNumber,
};
use codec::{Decode, Encode};
use frame_support::{Blake2_128Concat, RuntimeDebug, Twox64Concat};
use scale_info::TypeInfo;
use sp_core::storage::StorageKey;

/// Best known parachain head hash.
#[derive(Clone, Decode, Encode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct BestParaHeadHash {
	/// Number of relay block where this head has been read.
	///
	/// Parachain head is opaque to relay chain. So we can't simply decode it as a header of
	/// parachains and call `block_number()` on it. Instead, we're using the fact that parachain
	/// head is always built on top of previous head (because it is blockchain) and relay chain
	/// always imports parachain heads in order. What it means for us is that at any given
	/// **finalized** relay block `B`, head of parachain will be ancestor (or the same) of all
	/// parachain heads available at descendants of `B`.
	pub at_relay_block_number: RelayBlockNumber,
	/// Hash of parachain head.
	pub head_hash: ParaHash,
}

/// Returns runtime storage key of given parachain head at the source chain.
///
/// The head is stored by the `paras` pallet in the `Heads` map.
pub fn parachain_head_storage_key_at_source(
	paras_pallet_name: &str,
	para_id: ParaId,
) -> StorageKey {
	bp_runtime::storage_map_final_key::<Twox64Concat>(paras_pallet_name, "Heads", &para_id.encode())
}

/// Returns runtime storage key of best known parachain head at the target chain.
///
/// The head is stored by the `pallet-bridge-parachains` pallet in the `BestParaHeads` map.
pub fn best_parachain_head_hash_storage_key_at_target(
	bridge_parachains_pallet_name: &str,
	para_id: ParaId,
) -> StorageKey {
	bp_runtime::storage_map_final_key::<Blake2_128Concat>(
		bridge_parachains_pallet_name,
		"BestParaHeads",
		&para_id.encode(),
	)
}

/// Returns runtime storage key of the parachain head with given hash at the target chain.
///
/// The head is stored by the `pallet-bridge-parachains` pallet in the `ImportedParaHeads` map.
pub fn imported_parachain_head_storage_key_at_target(
	bridge_parachains_pallet_name: &str,
	para_id: ParaId,
	head_hash: ParaHash,
) -> StorageKey {
	bp_runtime::storage_double_map_final_key::<Blake2_128Concat, Blake2_128Concat>(
		bridge_parachains_pallet_name,
		"ImportedParaHeads",
		&para_id.encode(),
		&head_hash.encode(),
	)
}
