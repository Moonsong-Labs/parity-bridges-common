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

use super::{stagenet_messages_to_betanet, stagenet_relay_headers_to_betanet};

/// Stagenet-to-Betanet parachain sync description.
#[derive(Clone, Debug)]
pub struct StagenetToBetanet;

impl SubstrateParachainsPipeline for StagenetToBetanet {
	type SourceParachain = relay_moonbase_client::stagenet::Stagenet;
	type SourceRelayChain = relay_westend_client::Westend;
	type TargetChain = relay_moonbase_client::betanet::Betanet;

	type SubmitParachainHeadsCallBuilder = MoonriverToMoonbeamCallBuilder;
}

pub struct MoonriverToMoonbeamCallBuilder;
impl SubmitParachainHeadsCallBuilder<StagenetToBetanet> for MoonriverToMoonbeamCallBuilder {
	fn build_submit_parachain_heads_call(
		at_relay_block: HeaderIdOf<relay_kusama_client::Kusama>,
		parachains: Vec<(ParaId, ParaHash)>,
		parachain_heads_proof: ParaHeadsProof,
		_is_free_execution_expected: bool,
	) -> CallOf<relay_moonbase_client::betanet::Betanet> {
		relay_moonbase_client::RuntimeCall::BridgeParachains(
			relay_moonbase_client::BridgeParachainCall::submit_parachain_heads {
				at_relay_block: (at_relay_block.0, at_relay_block.1),
				parachains,
				parachain_heads_proof,
			},
		)
	}
}

pub struct CliBridge {}

impl ParachainToRelayHeadersCliBridge for CliBridge {
	type SourceRelay = relay_westend_client::Westend;
	type ParachainFinality = StagenetToBetanet;
	type RelayFinality = stagenet_relay_headers_to_betanet::StagenetFinality;
}

impl CliBridgeBase for CliBridge {
	type Source = relay_moonbase_client::stagenet::Stagenet;
	type Target = relay_moonbase_client::betanet::Betanet;
}

impl MessagesCliBridge for CliBridge {
	type MessagesLane = stagenet_messages_to_betanet::MessageLane;
}
