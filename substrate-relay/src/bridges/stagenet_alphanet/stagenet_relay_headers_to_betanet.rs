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

//! Stagenet-to-Betanet headers sync entrypoint.

use substrate_relay_helper::cli::bridge::{
	CliBridgeBase, RelayToRelayEquivocationDetectionCliBridge, RelayToRelayHeadersCliBridge,
};

use async_trait::async_trait;
use substrate_relay_helper::{
	equivocation::SubstrateEquivocationDetectionPipeline,
	finality::SubstrateFinalitySyncPipeline,
	finality_base::{engine::Grandpa as GrandpaFinalityEngine, SubstrateFinalityPipeline},
};

/// Description of Stagenet -> Betanet finalized headers bridge.
#[derive(Clone, Debug)]
pub struct StagenetFinality;

substrate_relay_helper::generate_submit_finality_proof_ex_call_builder!(
	StagenetFinality,
	SubmitFinalityProofCallBuilder,
	relay_moonbase_client::RuntimeCall::BridgeGrandpa,
	relay_moonbase_client::BridgeGrandpaCall::submit_finality_proof_ex
);

substrate_relay_helper::generate_report_equivocation_call_builder!(
	StagenetFinality,
	ReportEquivocationCallBuilder,
	relay_westend_client::RuntimeCall::Grandpa,
	relay_westend_client::GrandpaCall::report_equivocation
);

#[async_trait]
impl SubstrateFinalityPipeline for StagenetFinality {
	type SourceChain = relay_westend_client::Westend;
	type TargetChain = relay_moonbase_client::betanet::Betanet;

	type FinalityEngine = GrandpaFinalityEngine<Self::SourceChain>;
}

#[async_trait]
impl SubstrateFinalitySyncPipeline for StagenetFinality {
	type SubmitFinalityProofCallBuilder = SubmitFinalityProofCallBuilder;
}

#[async_trait]
impl SubstrateEquivocationDetectionPipeline for StagenetFinality {
	type ReportEquivocationCallBuilder = ReportEquivocationCallBuilder;
}

pub struct CliBridge {}

impl CliBridgeBase for CliBridge {
	type Source = relay_westend_client::Westend;
	type Target = relay_moonbase_client::betanet::Betanet;
}

impl RelayToRelayHeadersCliBridge for CliBridge {
	type Finality = StagenetFinality;
}

impl RelayToRelayEquivocationDetectionCliBridge for CliBridge {
	type Equivocation = StagenetFinality;
}
