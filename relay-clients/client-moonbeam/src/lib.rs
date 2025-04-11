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

//! Types used to connect to the Moonbeam parachain.

pub mod codegen_runtime;

use bp_polkadot_core::{SuffixedCommonTransactionExtension, SuffixedCommonTransactionExtensionExt};
use codec::Encode;
use relay_substrate_client::{
	calls::UtilityCall as MockUtilityCall, Chain, ChainWithBalances, ChainWithMessages,
	ChainWithRuntimeVersion, ChainWithTransactions, ChainWithUtilityPallet,
	Error as SubstrateError, MockedRuntimeUtilityPallet, SignParam, SimpleRuntimeVersion,
	UnderlyingChainProvider, UnsignedTransaction,
};
use sp_core::{keccak_256, storage::StorageKey, Pair};
use sp_runtime::{
	generic::SignedPayload,
	traits::{FakeDispatchable, IdentifyAccount},
};
use std::time::Duration;

pub use codegen_runtime::api::runtime_types;
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

pub type RuntimeCall = runtime_types::moonbeam_runtime::RuntimeCall;
pub type BridgeMessagesCall = runtime_types::pallet_bridge_messages::pallet::Call;
pub type BridgeGrandpaCall = runtime_types::pallet_bridge_grandpa::pallet::Call;
pub type BridgeParachainCall = runtime_types::pallet_bridge_parachains::pallet::Call;
type UncheckedExtrinsic =
	bp_moonbeam::UncheckedExtrinsic<RuntimeCall, TransactionExtension>;
type UtilityCall = runtime_types::pallet_utility::pallet::Call;

/// Polkadot chain definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moonbeam;

impl UnderlyingChainProvider for Moonbeam {
	type Chain = bp_moonbeam::Moonbeam;
}

impl Chain for Moonbeam {
	const NAME: &'static str = "Moonbeam";
	const BEST_FINALIZED_HEADER_ID_METHOD: &'static str =
		bp_moonbeam::BEST_FINALIZED_MOONBEAM_POLKADOT_HEADER_METHOD;
	const FREE_HEADERS_INTERVAL_METHOD: &'static str =
		bp_moonbeam::FREE_HEADERS_INTERVAL_FOR_MOONBEAM_POLKADOT_METHOD;
	const AVERAGE_BLOCK_INTERVAL: Duration = bp_moonbeam::AVERAGE_BLOCK_INTERVAL;

	type SignedBlock = bp_moonbeam::SignedBlock;
	type Call = RuntimeCall;
}

impl ChainWithBalances for Moonbeam {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		bp_moonbeam::AccountInfoStorageMapKeyProvider::final_key(account_id)
	}
}

impl From<MockUtilityCall<RuntimeCall>> for RuntimeCall {
	fn from(value: MockUtilityCall<RuntimeCall>) -> RuntimeCall {
		match value {
			MockUtilityCall::batch_all(calls) =>
				RuntimeCall::Utility(UtilityCall::batch_all { calls }),
		}
	}
}

impl ChainWithUtilityPallet for Moonbeam {
	type UtilityPallet = MockedRuntimeUtilityPallet<RuntimeCall>;
}

impl ChainWithTransactions for Moonbeam {
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

		let signature: bp_moonbeam::Signature = raw_payload.using_encoded(|payload| {
			// Moonbeam signer process hashes the message twice
			// 1. blake2_256
			// 2. keccak_256
			let mut h: [u8; 32] = [0u8; 32];
			h.copy_from_slice(keccak_256(payload).as_slice());
			param.signer.sign_prehashed(&h)
		}).into();
		let signer: sp_runtime::MultiSigner = param.signer.public().into();
		let (call, extra, _) = raw_payload.deconstruct();

		Ok(UncheckedExtrinsic::new_signed(
			call.deconstruct(),
			signer.into_account().into(),
			signature.into(),
			extra,
		))
	}
}

impl ChainWithMessages for Moonbeam {
	const TO_CHAIN_MESSAGE_DETAILS_METHOD: &'static str =
		bp_moonbeam::TO_MOONBEAM_POLKADOT_MESSAGE_DETAILS_METHOD;
	const FROM_CHAIN_MESSAGE_DETAILS_METHOD: &'static str =
		bp_moonbeam::FROM_MOONBEAM_POLKADOT_MESSAGE_DETAILS_METHOD;
}

impl ChainWithRuntimeVersion for Moonbeam {
	const RUNTIME_VERSION: Option<SimpleRuntimeVersion> =
		Some(SimpleRuntimeVersion { spec_version: 3_600, transaction_version: 3 });
}
