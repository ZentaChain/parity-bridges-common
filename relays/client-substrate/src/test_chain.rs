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

//! Pallet provides a set of guard functions that are running in background threads
//! and are aborting process if some condition fails.

//! Test chain implementation to use in tests.

#![cfg(any(feature = "test-helpers", test))]

use crate::{Chain, ChainWithBalances};
use frame_support::weights::{IdentityFee, Weight};
use std::time::Duration;

/// Chain that may be used in tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestChain;

impl bp_runtime::Chain for TestChain {
	type BlockNumber = u32;
	type Hash = sp_core::H256;
	type Hasher = sp_runtime::traits::BlakeTwo256;
	type Header = sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>;

	type AccountId = u32;
	type Balance = u32;
	type Index = u32;
	type Signature = sp_runtime::testing::TestSignature;

	fn max_extrinsic_size() -> u32 {
		unreachable!()
	}

	fn max_extrinsic_weight() -> Weight {
		unreachable!()
	}
}

impl Chain for TestChain {
	const NAME: &'static str = "Test";
	const TOKEN_ID: Option<&'static str> = None;
	const BEST_FINALIZED_HEADER_ID_METHOD: &'static str = "TestMethod";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_millis(0);
	const STORAGE_PROOF_OVERHEAD: u32 = 0;

	type SignedBlock = sp_runtime::generic::SignedBlock<
		sp_runtime::generic::Block<Self::Header, sp_runtime::OpaqueExtrinsic>,
	>;
	type Call = ();
	type WeightToFee = IdentityFee<u32>;
}

impl ChainWithBalances for TestChain {
	fn account_info_storage_key(_account_id: &u32) -> sp_core::storage::StorageKey {
		unreachable!()
	}
}
