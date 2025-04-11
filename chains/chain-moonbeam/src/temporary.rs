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

use frame_support::Blake2_128Concat;
use sp_core::{ecdsa, storage::StorageKey, RuntimeDebug, H160};
use codec::{Decode, Encode, DecodeWithMemTracking, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use sp_runtime::{generic, traits::BlakeTwo256};
use bp_runtime::{EncodedOrDecodedCall, StorageMapKeyProvider};

pub type AccountId = AccountId20;
/// Balance of an account.
pub type Balance = u128;
/// An index to a block.
pub type BlockNumber = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

#[derive(
	Eq, PartialEq, Copy, Clone, Encode, Decode, TypeInfo, MaxEncodedLen, Default, PartialOrd, Ord, DecodeWithMemTracking
)]
pub struct AccountId20(pub [u8; 20]);

impl_serde::impl_fixed_hash_serde!(AccountId20, 20);

#[cfg(feature = "std")]
impl std::fmt::Display for AccountId20 {
	//TODO This is a pretty quck-n-dirty implementation. Perhaps we should add
	// checksum casing here? I bet there is a crate for that.
	// Maybe this one https://github.com/miguelmota/rust-eth-checksum
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl core::fmt::Debug for AccountId20 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{:?}", H160(self.0))
	}
}

impl From<[u8; 20]> for AccountId20 {
	fn from(bytes: [u8; 20]) -> Self {
		Self(bytes)
	}
}

impl From<AccountId20> for [u8; 20] {
	fn from(value: AccountId20) -> Self {
		value.0
	}
}

// NOTE: the implementation is lossy, and is intended to be used
// only to convert from Polkadot accounts to AccountId20.
// See https://github.com/moonbeam-foundation/moonbeam/pull/2315#discussion_r1205830577
// DO NOT USE IT FOR ANYTHING ELSE.
impl From<[u8; 32]> for AccountId20 {
	fn from(bytes: [u8; 32]) -> Self {
		let mut buffer = [0u8; 20];
		buffer.copy_from_slice(&bytes[..20]);
		Self(buffer)
	}
}
impl From<sp_runtime::AccountId32> for AccountId20 {
	fn from(account: sp_runtime::AccountId32) -> Self {
		let bytes: &[u8; 32] = account.as_ref();
		Self::from(*bytes)
	}
}

impl From<H160> for AccountId20 {
	fn from(h160: H160) -> Self {
		Self(h160.0)
	}
}

impl From<AccountId20> for H160 {
	fn from(value: AccountId20) -> Self {
		H160(value.0)
	}
}

#[cfg(feature = "std")]
impl std::str::FromStr for AccountId20 {
	type Err = &'static str;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		H160::from_str(input)
			.map(Into::into)
			.map_err(|_| "invalid hex address.")
	}
}

impl From<ecdsa::Public> for AccountId20 {
	fn from(x: ecdsa::Public) -> Self {
		let decompressed = libsecp256k1::PublicKey::parse_slice(
			&x.0,
			Some(libsecp256k1::PublicKeyFormat::Compressed),
		)
		.expect("Wrong compressed public key provided")
		.serialize();
		let mut m = [0u8; 64];
		m.copy_from_slice(&decompressed[1..65]);
		let account = H160::from_slice(&Keccak256::digest(m).as_slice()[12..32]);
		account.into()
	}
}

#[derive(
	Eq, PartialEq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Serialize, Deserialize, DecodeWithMemTracking
)]
pub struct Signature(ecdsa::Signature);

impl From<ecdsa::Signature> for Signature {
	fn from(x: ecdsa::Signature) -> Self {
		Signature(x)
	}
}

impl From<sp_runtime::MultiSignature> for Signature {
	fn from(signature: sp_runtime::MultiSignature) -> Self {
		match signature {
			sp_runtime::MultiSignature::Ed25519(_) => {
				panic!("Ed25519 not supported for EthereumSignature")
			}
			sp_runtime::MultiSignature::Sr25519(_) => {
				panic!("Sr25519 not supported for EthereumSignature")
			}
			sp_runtime::MultiSignature::Ecdsa(sig) => Self(sig),
		}
	}
}


impl sp_runtime::traits::Verify for Signature {
	type Signer = EthereumSigner;
	fn verify<L: sp_runtime::traits::Lazy<[u8]>>(&self, mut msg: L, signer: &AccountId20) -> bool {
		let mut m = [0u8; 32];
		m.copy_from_slice(Keccak256::digest(msg.get()).as_slice());
		match sp_io::crypto::secp256k1_ecdsa_recover(self.0.as_ref(), &m) {
			Ok(pubkey) => {
				AccountId20(H160::from_slice(&Keccak256::digest(pubkey).as_slice()[12..32]).0)
					== *signer
			}
			Err(sp_io::EcdsaVerifyError::BadRS) => {
				log::error!(target: "evm", "Error recovering: Incorrect value of R or S");
				false
			}
			Err(sp_io::EcdsaVerifyError::BadV) => {
				log::error!(target: "evm", "Error recovering: Incorrect value of V");
				false
			}
			Err(sp_io::EcdsaVerifyError::BadSignature) => {
				log::error!(target: "evm", "Error recovering: Invalid signature");
				false
			}
		}
	}
}

/// Public key for an Ethereum / Moonbeam compatible account
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, sp_core::RuntimeDebug, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct EthereumSigner([u8; 20]);

impl sp_runtime::traits::IdentifyAccount for EthereumSigner {
	type AccountId = AccountId20;
	fn into_account(self) -> AccountId20 {
		AccountId20(self.0)
	}
}

impl From<[u8; 20]> for EthereumSigner {
	fn from(x: [u8; 20]) -> Self {
		EthereumSigner(x)
	}
}

impl From<ecdsa::Public> for EthereumSigner {
	fn from(x: ecdsa::Public) -> Self {
		let decompressed = libsecp256k1::PublicKey::parse_slice(
			&x.0,
			Some(libsecp256k1::PublicKeyFormat::Compressed),
		)
		.expect("Wrong compressed public key provided")
		.serialize();
		let mut m = [0u8; 64];
		m.copy_from_slice(&decompressed[1..65]);
		let account = H160::from_slice(&Keccak256::digest(m).as_slice()[12..32]);
		EthereumSigner(account.into())
	}
}

impl From<libsecp256k1::PublicKey> for EthereumSigner {
	fn from(x: libsecp256k1::PublicKey) -> Self {
		let mut m = [0u8; 64];
		m.copy_from_slice(&x.serialize()[1..65]);
		let account = H160::from_slice(&Keccak256::digest(m).as_slice()[12..32]);
		EthereumSigner(account.into())
	}
}

#[cfg(feature = "std")]
impl std::fmt::Display for EthereumSigner {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(fmt, "ethereum signature: {:?}", H160::from_slice(&self.0))
	}
}

/// Provides a storage key for account data.
///
/// We need to use this approach when we don't have access to the runtime.
/// The equivalent command to invoke in case full `Runtime` is known is this:
/// `let key = frame_system::Account::<Runtime>::storage_map_final_key(&account_id);`
pub struct AccountInfoStorageMapKeyProvider;

impl StorageMapKeyProvider for AccountInfoStorageMapKeyProvider {
	const MAP_NAME: &'static str = "Account";
	type Hasher = Blake2_128Concat;
	type Key = AccountId;
	// This should actually be `AccountInfo`, but we don't use this property in order to decode the
	// data. So we use `Vec<u8>` as if we would work with encoded data.
	type Value = Vec<u8>;
}

impl AccountInfoStorageMapKeyProvider {
	/// Name of the system pallet.
	const PALLET_NAME: &'static str = "System";

	/// Return storage key for given account data.
	pub fn final_key(id: &AccountId) -> StorageKey {
		<Self as StorageMapKeyProvider>::final_key(Self::PALLET_NAME, id)
	}
}

/// Unchecked Extrinsic type.
pub type UncheckedExtrinsic<Call, SignedExt> =
	generic::UncheckedExtrinsic<AccountId, EncodedOrDecodedCall<Call>, Signature, SignedExt>;
