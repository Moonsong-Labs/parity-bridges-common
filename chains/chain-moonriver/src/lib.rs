// Copyright 2025 Moonbeam foundation
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! # Moonriver bridge primitives

#![cfg_attr(not(feature = "std"), no_std)]

pub use bp_bridge_hub_cumulus::{
	BlockLength, BlockWeights, Hasher, Nonce, SignedBlock, AVERAGE_BLOCK_INTERVAL,
	MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX, MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX,
};
use bp_messages::{ChainWithMessages, MessageNonce};

pub use bp_moonbeam::{
	AccountId, AccountInfoStorageMapKeyProvider, Balance, BlockNumber, Hash, Header, Signature,
	UncheckedExtrinsic,
};

use bp_runtime::{
	decl_bridge_finality_runtime_apis, decl_bridge_messages_runtime_apis, Chain, ChainId, Parachain,
};
use frame_support::{dispatch::DispatchClass, weights::Weight};
use sp_runtime::StateVersion;

/// Moonriver parachain.
pub struct Moonriver;

impl Chain for Moonriver {
	const ID: ChainId = *b"mnrv";

	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hasher = Hasher;
	type Header = Header;

	type AccountId = AccountId;
	type Balance = Balance;
	type Nonce = Nonce;
	type Signature = Signature;

	const STATE_VERSION: StateVersion = StateVersion::V1;

	fn max_extrinsic_size() -> u32 {
		*BlockLength::get().max.get(DispatchClass::Normal)
	}

	fn max_extrinsic_weight() -> Weight {
		BlockWeights::get()
			.get(DispatchClass::Normal)
			.max_extrinsic
			.unwrap_or(Weight::MAX)
	}
}

impl Parachain for Moonriver {
	const PARACHAIN_ID: u32 = MOONRIVER_KUSAMA_PARACHAIN_ID;
	const MAX_HEADER_SIZE: u32 = 4_096;
}

impl ChainWithMessages for Moonriver {
	const WITH_CHAIN_MESSAGES_PALLET_NAME: &'static str =
		WITH_MOONRIVER_KUSAMA_MESSAGES_PALLET_NAME;

	const MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX: MessageNonce =
		MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX;
	const MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX: MessageNonce =
		MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX;
}

/// Identifier of Moonbeam parachain in the Kusama relay chain.
pub const MOONRIVER_KUSAMA_PARACHAIN_ID: u32 = 2023;

/// Name of the With-MoonriverKusama messages pallet instance that is deployed at bridged chains.
pub const WITH_MOONRIVER_KUSAMA_MESSAGES_PALLET_NAME: &str = "BridgeKusamaMessages";

/// Name of the With-MoonriverKusama bridge-relayers pallet instance that is deployed at bridged
/// chains.
pub const WITH_MOONRIVER_KUSAMA_RELAYERS_PALLET_NAME: &str = "BridgeRelayers";

/// Bridge lane identifier.
pub type LaneId = bp_messages::HashedLaneId;

decl_bridge_finality_runtime_apis!(moonriver_kusama);
decl_bridge_messages_runtime_apis!(moonriver_kusama, LaneId);

// TODO: Update values
frame_support::parameter_types! {
	/// The XCM fee that is paid for executing XCM program (with `ExportMessage` instruction) at the Kusama
	/// BridgeHub.
	/// (initially was calculated by test `BridgeHubKusama::can_calculate_weight_for_paid_export_message_with_reserve_transfer` + `33%`)
	pub const BaseXcmFeeInMovr: u128 = 601_115_666;

	/// Transaction fee that is paid at the Kusama BridgeHub for delivering single inbound message.
	/// (initially was calculated by test `BridgeHubKusama::can_calculate_fee_for_complex_message_delivery_transaction` + `33%`)
	pub const BaseDeliveryFeeInMovr: u128 = 3_142_112_953;

	/// Transaction fee that is paid at the Kusama BridgeHub for delivering single outbound message confirmation.
	/// (initially was calculated by test `BridgeHubKusama::can_calculate_fee_for_complex_message_confirmation_transaction` + `33%`)
	pub const BaseConfirmationFeeInMovr: u128 = 575_036_072;
}
