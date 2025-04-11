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

//! Moonbeam-to-Moonriver messages sync entrypoint.

use relay_moonbeam_client::Moonbeam;
use relay_moonriver_client::Moonriver;
use substrate_relay_helper::{messages::SubstrateMessageLane, UtilityPalletBatchCallBuilder};

substrate_relay_helper::generate_receive_message_proof_call_builder!(
	MoonbeamMessagesToMoonriverMessageLane,
	MoonbeamMessagesToMoonriverMessageLaneReceiveMessagesProofCallBuilder,
	relay_moonriver_client::RuntimeCall::BridgePolkadotMessages,
	relay_moonriver_client::BridgeMessagesCall::receive_messages_proof
);

substrate_relay_helper::generate_receive_message_delivery_proof_call_builder!(
	MoonbeamMessagesToMoonriverMessageLane,
	MoonbeamMessagesToMoonriverMessageLaneReceiveMessagesDeliveryProofCallBuilder,
	relay_moonbeam_client::RuntimeCall::BridgeKusamaMessages,
	relay_moonbeam_client::BridgeMessagesCall::receive_messages_delivery_proof
);

/// Moonbeam-to-Moonriver messages lane.
#[derive(Clone, Debug)]
pub struct MoonbeamMessagesToMoonriverMessageLane;

impl SubstrateMessageLane for MoonbeamMessagesToMoonriverMessageLane {
	type SourceChain = Moonbeam;
	type TargetChain = Moonriver;

	type LaneId = bp_messages::LegacyLaneId;

	type ReceiveMessagesProofCallBuilder =
		MoonbeamMessagesToMoonriverMessageLaneReceiveMessagesProofCallBuilder;
	type ReceiveMessagesDeliveryProofCallBuilder =
		MoonbeamMessagesToMoonriverMessageLaneReceiveMessagesDeliveryProofCallBuilder;

	type SourceBatchCallBuilder = UtilityPalletBatchCallBuilder<Moonbeam>;
	type TargetBatchCallBuilder = UtilityPalletBatchCallBuilder<Moonriver>;
}
