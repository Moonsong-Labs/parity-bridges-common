// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Declaration of all bridges between Stagenet relay and Alphanet relay.

// Betanet <> Stagenet modules
pub mod betanet_messages_to_stagenet;
pub mod betanet_parachains_to_stagenet;
pub mod betanet_relay_headers_to_stagenet;
pub mod stagenet_messages_to_betanet;
pub mod stagenet_parachains_to_betanet;
pub mod stagenet_relay_headers_to_betanet;
