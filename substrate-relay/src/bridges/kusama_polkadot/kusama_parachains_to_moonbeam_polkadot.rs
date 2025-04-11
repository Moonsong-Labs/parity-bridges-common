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

//! Moonriver-to-Moonbeam parachains sync entrypoint.

use bp_polkadot_core::parachains::{ParaHash, ParaHeadsProof, ParaId};
use relay_substrate_client::{CallOf, HeaderIdOf};
use substrate_relay_helper::{
	cli::bridge::{CliBridgeBase, MessagesCliBridge, ParachainToRelayHeadersCliBridge},
	parachains::{SubmitParachainHeadsCallBuilder, SubstrateParachainsPipeline},
};

/// Moonriver-to-Moonbeam parachain sync description.
#[derive(Clone, Debug)]
pub struct MoonriverToMoonbeam;

impl SubstrateParachainsPipeline for MoonriverToMoonbeam {
	type SourceParachain = relay_moonriver_client::Moonriver;
	type SourceRelayChain = relay_kusama_client::Kusama;
	type TargetChain = relay_moonbeam_client::Moonbeam;

	type SubmitParachainHeadsCallBuilder = MoonriverToMoonbeamCallBuilder;
}

pub struct MoonriverToMoonbeamCallBuilder;
impl SubmitParachainHeadsCallBuilder<MoonriverToMoonbeam> for MoonriverToMoonbeamCallBuilder {
	fn build_submit_parachain_heads_call(
		at_relay_block: HeaderIdOf<relay_kusama_client::Kusama>,
		parachains: Vec<(ParaId, ParaHash)>,
		parachain_heads_proof: ParaHeadsProof,
		_is_free_execution_expected: bool,
	) -> CallOf<relay_moonbeam_client::Moonbeam> {
		relay_moonbeam_client::RuntimeCall::BridgeKusamaParachains(
			relay_moonbeam_client::BridgeParachainCall::submit_parachain_heads {
				at_relay_block: (at_relay_block.0, at_relay_block.1),
				parachains,
				parachain_heads_proof,
			},
		)
	}
}

///  Moonriver-to-Moonbeam parachain sync description for the CLI.
pub struct MoonriverToMoonbeamCliBridge {}

impl ParachainToRelayHeadersCliBridge for MoonriverToMoonbeamCliBridge {
	type SourceRelay = relay_kusama_client::Kusama;
	type ParachainFinality = MoonriverToMoonbeam;
	type RelayFinality =
		crate::bridges::kusama_polkadot::kusama_headers_to_moonbeam::KusamaFinalityToMoonbeam;
}

impl CliBridgeBase for MoonriverToMoonbeamCliBridge {
	type Source = relay_moonriver_client::Moonriver;
	type Target = relay_moonbeam_client::Moonbeam;
}

impl MessagesCliBridge for MoonriverToMoonbeamCliBridge {
	type MessagesLane =
	crate::bridges::kusama_polkadot::moonriver_kusama_messages_to_moonbeam_polkadot::MoonriverMessagesToMoonbeamMessageLane;
}
