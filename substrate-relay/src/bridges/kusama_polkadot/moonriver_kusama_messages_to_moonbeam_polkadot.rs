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

//! Moonriver-to-Moonbeam messages sync entrypoint.

use relay_moonbeam_client::Moonbeam;
use relay_moonriver_client::Moonriver;
use substrate_relay_helper::{
	cli::bridge::{CliBridgeBase, MessagesCliBridge},
	messages::SubstrateMessageLane,
	UtilityPalletBatchCallBuilder,
};

/// Moonriver-to-Moonbeam messages bridge.
pub struct MoonriverToMoonbeamMessagesCliBridge {}

impl CliBridgeBase for MoonriverToMoonbeamMessagesCliBridge {
	type Source = Moonriver;
	type Target = Moonbeam;
}

impl MessagesCliBridge for MoonriverToMoonbeamMessagesCliBridge {
	type MessagesLane = MoonriverMessagesToMoonbeamMessageLane;
}

substrate_relay_helper::generate_receive_message_proof_call_builder!(
	MoonriverMessagesToMoonbeamMessageLane,
	MoonriverMessagesToMoonbeamMessageLaneReceiveMessagesProofCallBuilder,
	relay_moonbeam_client::RuntimeCall::BridgeKusamaMessages,
	relay_moonbeam_client::BridgeMessagesCall::receive_messages_proof
);

substrate_relay_helper::generate_receive_message_delivery_proof_call_builder!(
	MoonriverMessagesToMoonbeamMessageLane,
	MoonriverMessagesToMoonbeamMessageLaneReceiveMessagesDeliveryProofCallBuilder,
	relay_moonriver_client::RuntimeCall::BridgePolkadotMessages,
	relay_moonriver_client::BridgeMessagesCall::receive_messages_delivery_proof
);

/// Moonriver-to-Moonbeam messages lane.
#[derive(Clone, Debug)]
pub struct MoonriverMessagesToMoonbeamMessageLane;

impl SubstrateMessageLane for MoonriverMessagesToMoonbeamMessageLane {
	type SourceChain = Moonriver;
	type TargetChain = Moonbeam;

	type LaneId = bp_messages::HashedLaneId;

	type ReceiveMessagesProofCallBuilder =
		MoonriverMessagesToMoonbeamMessageLaneReceiveMessagesProofCallBuilder;
	type ReceiveMessagesDeliveryProofCallBuilder =
		MoonriverMessagesToMoonbeamMessageLaneReceiveMessagesDeliveryProofCallBuilder;

	type SourceBatchCallBuilder = UtilityPalletBatchCallBuilder<Moonriver>;
	type TargetBatchCallBuilder = UtilityPalletBatchCallBuilder<Moonbeam>;
}
