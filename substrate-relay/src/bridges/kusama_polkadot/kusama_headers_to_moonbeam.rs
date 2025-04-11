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

//! Kusama-to-Moonbeam headers sync entrypoint.

use substrate_relay_helper::cli::bridge::{
	CliBridgeBase, RelayToRelayEquivocationDetectionCliBridge, RelayToRelayHeadersCliBridge,
};

use async_trait::async_trait;
use substrate_relay_helper::{
	equivocation::SubstrateEquivocationDetectionPipeline,
	finality::SubstrateFinalitySyncPipeline,
	finality_base::{engine::Grandpa as GrandpaFinalityEngine, SubstrateFinalityPipeline},
};

/// Description of Kusama -> Moonbeam finalized headers bridge.
#[derive(Clone, Debug)]
pub struct KusamaFinalityToMoonbeam;

substrate_relay_helper::generate_submit_finality_proof_ex_call_builder!(
	KusamaFinalityToMoonbeam,
	SubmitFinalityProofCallBuilder,
	relay_moonbeam_client::RuntimeCall::BridgeKusamaGrandpa,
	relay_moonbeam_client::BridgeGrandpaCall::submit_finality_proof_ex
);

substrate_relay_helper::generate_report_equivocation_call_builder!(
	KusamaFinalityToMoonbeam,
	ReportEquivocationCallBuilder,
	relay_kusama_client::RuntimeCall::Grandpa,
	relay_kusama_client::GrandpaCall::report_equivocation
);

#[async_trait]
impl SubstrateFinalityPipeline for KusamaFinalityToMoonbeam {
	type SourceChain = relay_kusama_client::Kusama;
	type TargetChain = relay_moonbeam_client::Moonbeam;

	type FinalityEngine = GrandpaFinalityEngine<Self::SourceChain>;
}

#[async_trait]
impl SubstrateFinalitySyncPipeline for KusamaFinalityToMoonbeam {
	type SubmitFinalityProofCallBuilder = SubmitFinalityProofCallBuilder;
}

#[async_trait]
impl SubstrateEquivocationDetectionPipeline for KusamaFinalityToMoonbeam {
	type ReportEquivocationCallBuilder = ReportEquivocationCallBuilder;
}

/// Kusama to Moonbeam bridge definition.
pub struct KusamaToMoonbeamCliBridge {}

impl CliBridgeBase for KusamaToMoonbeamCliBridge {
	type Source = relay_kusama_client::Kusama;
	type Target = relay_moonbeam_client::Moonbeam;
}

impl RelayToRelayHeadersCliBridge for KusamaToMoonbeamCliBridge {
	type Finality = KusamaFinalityToMoonbeam;
}

impl RelayToRelayEquivocationDetectionCliBridge for KusamaToMoonbeamCliBridge {
	type Equivocation = KusamaFinalityToMoonbeam;
}
