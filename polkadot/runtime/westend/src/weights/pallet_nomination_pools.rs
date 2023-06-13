// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_nomination_pools`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_nomination_pools
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_nomination_pools`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nomination_pools::WeightInfo for WeightInfo<T> {
	/// Storage: NominationPools MinJoinBond (r:1 w:0)
	/// Proof: NominationPools MinJoinBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	/// Proof: NominationPools MaxPoolMembersPerPool (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	/// Proof: NominationPools MaxPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// Proof: NominationPools CounterForPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:3 w:3)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn join() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3281`
		//  Estimated: `8877`
		// Minimum execution time: 176_369_000 picoseconds.
		Weight::from_parts(177_523_000, 0)
			.saturating_add(Weight::from_parts(0, 8877))
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:3 w:3)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn bond_extra_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3291`
		//  Estimated: `8877`
		// Minimum execution time: 172_285_000 picoseconds.
		Weight::from_parts(174_193_000, 0)
			.saturating_add(Weight::from_parts(0, 8877))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: NominationPools ClaimPermissions (r:1 w:0)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:2 w:2)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn bond_extra_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3186`
		//  Estimated: `8799`
		// Minimum execution time: 199_828_000 picoseconds.
		Weight::from_parts(200_942_000, 0)
			.saturating_add(Weight::from_parts(0, 8799))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: NominationPools ClaimPermissions (r:1 w:0)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1137`
		//  Estimated: `4182`
		// Minimum execution time: 74_431_000 picoseconds.
		Weight::from_parts(75_741_000, 0)
			.saturating_add(Weight::from_parts(0, 4182))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking MinNominatorBond (r:1 w:0)
	/// Proof: Staking MinNominatorBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:3 w:3)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools SubPoolsStorage (max_values: None, max_size: Some(261), added: 2736, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools CounterForSubPoolsStorage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn unbond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3560`
		//  Estimated: `8877`
		// Minimum execution time: 158_030_000 picoseconds.
		Weight::from_parts(158_878_000, 0)
			.saturating_add(Weight::from_parts(0, 8877))
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn pool_withdraw_unbonded(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1614`
		//  Estimated: `4764`
		// Minimum execution time: 59_728_000 picoseconds.
		Weight::from_parts(60_957_789, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 741
			.saturating_add(Weight::from_parts(6_046, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools SubPoolsStorage (max_values: None, max_size: Some(261), added: 2736, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// Proof: NominationPools CounterForPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools ClaimPermissions (r:0 w:1)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2042`
		//  Estimated: `4764`
		// Minimum execution time: 121_936_000 picoseconds.
		Weight::from_parts(123_798_311, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 1_169
			.saturating_add(Weight::from_parts(21_320, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools SubPoolsStorage (max_values: None, max_size: Some(261), added: 2736, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:1)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:1 w:0)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking Validators (r:1 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// Proof: NominationPools CounterForPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	/// Proof: NominationPools ReversePoolIdLookup (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	/// Proof: NominationPools CounterForReversePoolIdLookup (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	/// Proof: NominationPools CounterForRewardPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools CounterForSubPoolsStorage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools Metadata (r:1 w:1)
	/// Proof: NominationPools Metadata (max_values: None, max_size: Some(270), added: 2745, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	/// Proof: NominationPools CounterForBondedPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Payee (r:0 w:1)
	/// Proof: Staking Payee (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: NominationPools ClaimPermissions (r:0 w:1)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2398`
		//  Estimated: `6196`
		// Minimum execution time: 203_824_000 picoseconds.
		Weight::from_parts(206_388_841, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			// Standard Error: 2_817
			.saturating_add(Weight::from_parts(13_627, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(21))
			.saturating_add(T::DbWeight::get().writes(18))
	}
	/// Storage: NominationPools LastPoolId (r:1 w:1)
	/// Proof: NominationPools LastPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking MinNominatorBond (r:1 w:0)
	/// Proof: Staking MinNominatorBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools MinCreateBond (r:1 w:0)
	/// Proof: NominationPools MinCreateBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools MinJoinBond (r:1 w:0)
	/// Proof: NominationPools MinJoinBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPools (r:1 w:0)
	/// Proof: NominationPools MaxPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	/// Proof: NominationPools CounterForBondedPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	/// Proof: NominationPools MaxPoolMembersPerPool (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	/// Proof: NominationPools MaxPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// Proof: NominationPools CounterForPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:1)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	/// Proof: NominationPools CounterForRewardPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	/// Proof: NominationPools ReversePoolIdLookup (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	/// Proof: NominationPools CounterForReversePoolIdLookup (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Payee (r:0 w:1)
	/// Proof: Staking Payee (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1222`
		//  Estimated: `6196`
		// Minimum execution time: 177_985_000 picoseconds.
		Weight::from_parts(179_504_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(22))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking MinNominatorBond (r:1 w:0)
	/// Proof: Staking MinNominatorBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:1)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking MaxNominatorsCount (r:1 w:0)
	/// Proof: Staking MaxNominatorsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:17 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:1 w:1)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking CounterForNominators (r:1 w:1)
	/// Proof: Staking CounterForNominators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 16]`.
	fn nominate(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1779`
		//  Estimated: `4556 + n * (2520 ±0)`
		// Minimum execution time: 64_575_000 picoseconds.
		Weight::from_parts(64_712_179, 0)
			.saturating_add(Weight::from_parts(0, 4556))
			// Standard Error: 4_612
			.saturating_add(Weight::from_parts(1_300_327, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(Weight::from_parts(0, 2520).saturating_mul(n.into()))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	fn set_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1367`
		//  Estimated: `4556`
		// Minimum execution time: 35_436_000 picoseconds.
		Weight::from_parts(35_843_000, 0)
			.saturating_add(Weight::from_parts(0, 4556))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools Metadata (r:1 w:1)
	/// Proof: NominationPools Metadata (max_values: None, max_size: Some(270), added: 2745, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForMetadata (r:1 w:1)
	/// Proof: NominationPools CounterForMetadata (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 256]`.
	fn set_metadata(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `497`
		//  Estimated: `3735`
		// Minimum execution time: 14_017_000 picoseconds.
		Weight::from_parts(14_595_057, 0)
			.saturating_add(Weight::from_parts(0, 3735))
			// Standard Error: 77
			.saturating_add(Weight::from_parts(1_065, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: NominationPools MinJoinBond (r:0 w:1)
	/// Proof: NominationPools MinJoinBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembers (r:0 w:1)
	/// Proof: NominationPools MaxPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembersPerPool (r:0 w:1)
	/// Proof: NominationPools MaxPoolMembersPerPool (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MinCreateBond (r:0 w:1)
	/// Proof: NominationPools MinCreateBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:0 w:1)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPools (r:0 w:1)
	/// Proof: NominationPools MaxPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_configs() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_932_000 picoseconds.
		Weight::from_parts(6_145_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	fn update_roles() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `497`
		//  Estimated: `3685`
		// Minimum execution time: 18_860_000 picoseconds.
		Weight::from_parts(19_156_000, 0)
			.saturating_add(Weight::from_parts(0, 3685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:1 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:1)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking CounterForNominators (r:1 w:1)
	/// Proof: Staking CounterForNominators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:1 w:1)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn chill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1942`
		//  Estimated: `4556`
		// Minimum execution time: 62_812_000 picoseconds.
		Weight::from_parts(63_227_000, 0)
			.saturating_add(Weight::from_parts(0, 4556))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn set_commission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `736`
		//  Estimated: `3685`
		// Minimum execution time: 31_983_000 picoseconds.
		Weight::from_parts(32_552_000, 0)
			.saturating_add(Weight::from_parts(0, 3685))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	fn set_commission_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `537`
		//  Estimated: `3685`
		// Minimum execution time: 18_135_000 picoseconds.
		Weight::from_parts(18_821_000, 0)
			.saturating_add(Weight::from_parts(0, 3685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	fn set_commission_change_rate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `497`
		//  Estimated: `3685`
		// Minimum execution time: 18_969_000 picoseconds.
		Weight::from_parts(19_175_000, 0)
			.saturating_add(Weight::from_parts(0, 3685))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:0)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools ClaimPermissions (r:1 w:1)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	fn set_claim_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `508`
		//  Estimated: `4182`
		// Minimum execution time: 13_943_000 picoseconds.
		Weight::from_parts(14_374_000, 0)
			.saturating_add(Weight::from_parts(0, 4182))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_commission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `934`
		//  Estimated: `3685`
		// Minimum execution time: 61_658_000 picoseconds.
		Weight::from_parts(62_636_000, 0)
			.saturating_add(Weight::from_parts(0, 3685))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
