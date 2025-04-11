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

//! Stagenet-to-Betanet messages sync entrypoint.

use relay_moonbase_client::{betanet::Betanet, stagenet::Stagenet};
use substrate_relay_helper::{messages::SubstrateMessageLane, UtilityPalletBatchCallBuilder};

substrate_relay_helper::generate_receive_message_proof_call_builder!(
	MessageLane,
	MessageLaneReceiveMessagesProofCallBuilder,
	relay_moonbase_client::RuntimeCall::BridgeMessages,
	relay_moonbase_client::BridgeMessagesCall::receive_messages_proof
);

substrate_relay_helper::generate_receive_message_delivery_proof_call_builder!(
	MessageLane,
	MessageLaneReceiveMessagesDeliveryProofCallBuilder,
	relay_moonbase_client::RuntimeCall::BridgeMessages,
	relay_moonbase_client::BridgeMessagesCall::receive_messages_delivery_proof
);

/// Stagenet-to-Betanet messages lane.
#[derive(Clone, Debug)]
pub struct MessageLane;

impl SubstrateMessageLane for MessageLane {
	type SourceChain = Stagenet;
	type TargetChain = Betanet;

	type LaneId = bp_moonbase::LaneId;

	type ReceiveMessagesProofCallBuilder = MessageLaneReceiveMessagesProofCallBuilder;
	type ReceiveMessagesDeliveryProofCallBuilder =
		MessageLaneReceiveMessagesDeliveryProofCallBuilder;

	type SourceBatchCallBuilder = UtilityPalletBatchCallBuilder<Stagenet>;
	type TargetBatchCallBuilder = UtilityPalletBatchCallBuilder<Betanet>;
}
