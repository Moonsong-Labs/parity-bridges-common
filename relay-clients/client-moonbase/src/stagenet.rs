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

//! Types used to connect to the Stagenet parachain.

use bp_polkadot_core::{SuffixedCommonTransactionExtension, SuffixedCommonTransactionExtensionExt};
use codec::Encode;
use relay_substrate_client::{
	Chain, ChainWithBalances, ChainWithMessages, ChainWithRuntimeVersion, ChainWithTransactions,
	ChainWithUtilityPallet, Error as SubstrateError, MockedRuntimeUtilityPallet, SignParam,
	SimpleRuntimeVersion, UnderlyingChainProvider, UnsignedTransaction,
};
use sp_core::{keccak_256, storage::StorageKey, Pair};
use sp_runtime::{
	generic::SignedPayload,
	traits::{FakeDispatchable, IdentifyAccount},
};
use std::time::Duration;

pub use crate::codegen_runtime::api::runtime_types;
use runtime_types::frame_metadata_hash_extension::Mode;

use bp_runtime::extensions::{
	BridgeRejectObsoleteHeadersAndMessages, GenericTransactionExtensionSchema,
	RefundBridgedParachainMessagesSchema,
};

pub type CheckMetadataHash = GenericTransactionExtensionSchema<Mode, Option<[u8; 32]>>;

pub type TransactionExtension = SuffixedCommonTransactionExtension<(
	BridgeRejectObsoleteHeadersAndMessages,
	RefundBridgedParachainMessagesSchema,
	CheckMetadataHash,
)>;

pub type RuntimeCall = runtime_types::moonbase_runtime::RuntimeCall;
pub type BridgeMessagesCall = runtime_types::pallet_bridge_messages::pallet::Call;
pub type BridgeGrandpaCall = runtime_types::pallet_bridge_grandpa::pallet::Call;
pub type BridgeParachainCall = runtime_types::pallet_bridge_parachains::pallet::Call;
type UncheckedExtrinsic = bp_moonbase::UncheckedExtrinsic<RuntimeCall, TransactionExtension>;

/// Polkadot chain definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stagenet;

impl UnderlyingChainProvider for Stagenet {
	type Chain = bp_moonbase::stagenet::Stagenet;
}

impl Chain for Stagenet {
	const NAME: &'static str = "Stagenet";
	const BEST_FINALIZED_HEADER_ID_METHOD: &'static str =
		bp_moonbase::BEST_FINALIZED_MOONBASE_WESTEND_HEADER_METHOD;
	const FREE_HEADERS_INTERVAL_METHOD: &'static str =
		bp_moonbase::FREE_HEADERS_INTERVAL_FOR_MOONBASE_WESTEND_METHOD;
	const AVERAGE_BLOCK_INTERVAL: Duration = bp_moonbase::AVERAGE_BLOCK_INTERVAL;

	type SignedBlock = bp_moonbase::SignedBlock;
	type Call = RuntimeCall;
}

impl ChainWithBalances for Stagenet {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		bp_moonbase::AccountInfoStorageMapKeyProvider::final_key(account_id)
	}
}

impl ChainWithUtilityPallet for Stagenet {
	type UtilityPallet = MockedRuntimeUtilityPallet<RuntimeCall>;
}

impl ChainWithTransactions for Stagenet {
	type AccountKeyPair = sp_core::ecdsa::Pair;
	type SignedTransaction = UncheckedExtrinsic;

	fn sign_transaction(
		param: SignParam<Self>,
		unsigned: UnsignedTransaction<Self>,
	) -> Result<Self::SignedTransaction, SubstrateError> {
		let raw_payload = SignedPayload::new(
			FakeDispatchable::from(unsigned.call),
			TransactionExtension::from_params(
				param.spec_version,
				param.transaction_version,
				unsigned.era,
				param.genesis_hash,
				unsigned.nonce,
				unsigned.tip,
				(((), (), Mode::Disabled), ((), (), None)),
			),
		)?;

		let signature: bp_moonbase::Signature = raw_payload
			.using_encoded(|payload| {
				// Moonbeam signer hashes the message twice
				// 1. blake2_256
				// 2. keccak_256
				let mut h: [u8; 32] = [0u8; 32];
				h.copy_from_slice(keccak_256(payload).as_slice());
				param.signer.sign_prehashed(&h)
			})
			.into();
		let signer = param.signer.public();
		let (call, extra, _) = raw_payload.deconstruct();

		Ok(Self::SignedTransaction::new_signed(
			call.deconstruct(),
			signer.into_account().into(),
			signature,
			extra,
		))
	}
}

impl ChainWithMessages for Stagenet {
	const TO_CHAIN_MESSAGE_DETAILS_METHOD: &'static str =
		bp_moonbase::TO_MOONBASE_WESTEND_MESSAGE_DETAILS_METHOD;
	const FROM_CHAIN_MESSAGE_DETAILS_METHOD: &'static str =
		bp_moonbase::FROM_MOONBASE_WESTEND_MESSAGE_DETAILS_METHOD;
}

impl ChainWithRuntimeVersion for Stagenet {
	const RUNTIME_VERSION: Option<SimpleRuntimeVersion> =
		Some(SimpleRuntimeVersion { spec_version: 3_700, transaction_version: 3 });
}
