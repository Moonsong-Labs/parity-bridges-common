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

//! Moonbeam-to-Moonriver parachains sync entrypoint.

use bp_polkadot_core::parachains::{ParaHash, ParaHeadsProof, ParaId};
use relay_substrate_client::{CallOf, HeaderIdOf};
use substrate_relay_helper::{
	cli::bridge::{CliBridgeBase, MessagesCliBridge, ParachainToRelayHeadersCliBridge},
	parachains::{SubmitParachainHeadsCallBuilder, SubstrateParachainsPipeline},
};

/// Moonbeam-to-Moonriver parachain sync description.
#[derive(Clone, Debug)]
pub struct MoonbeamToMoonriver;

impl SubstrateParachainsPipeline for MoonbeamToMoonriver {
	type SourceParachain = relay_moonbeam_client::Moonbeam;
	type SourceRelayChain = relay_polkadot_client::Polkadot;
	type TargetChain = relay_moonriver_client::Moonriver;

	type SubmitParachainHeadsCallBuilder = MoonbeamToMoonriverCallBuilder;
}

pub struct MoonbeamToMoonriverCallBuilder;
impl SubmitParachainHeadsCallBuilder<MoonbeamToMoonriver> for MoonbeamToMoonriverCallBuilder {
	fn build_submit_parachain_heads_call(
		at_relay_block: HeaderIdOf<relay_polkadot_client::Polkadot>,
		parachains: Vec<(ParaId, ParaHash)>,
		parachain_heads_proof: ParaHeadsProof,
		_is_free_execution_expected: bool,
	) -> CallOf<relay_moonriver_client::Moonriver> {
		relay_moonriver_client::RuntimeCall::BridgePolkadotParachains(
			relay_moonriver_client::BridgeParachainCall::submit_parachain_heads {
				at_relay_block: (at_relay_block.0, at_relay_block.1),
				parachains,
				parachain_heads_proof,
			},
		)
	}
}

/// Moonbeam-to-Moonriver parachain sync description for the CLI.
pub struct MoonbeamToMoonriverCliBridge {}

impl ParachainToRelayHeadersCliBridge for MoonbeamToMoonriverCliBridge {
	type SourceRelay = relay_polkadot_client::Polkadot;
	type ParachainFinality = MoonbeamToMoonriver;
	type RelayFinality =
		crate::bridges::kusama_polkadot::polkadot_headers_to_moonriver::PolkadotFinalityToMoonriver;
}

impl CliBridgeBase for MoonbeamToMoonriverCliBridge {
	type Source = relay_moonbeam_client::Moonbeam;
	type Target = relay_moonriver_client::Moonriver;
}

impl MessagesCliBridge for MoonbeamToMoonriverCliBridge {
	type MessagesLane =
	crate::bridges::kusama_polkadot::moonbeam_polkadot_messages_to_moonriver_kusama::MoonbeamMessagesToMoonriverMessageLane;
}
