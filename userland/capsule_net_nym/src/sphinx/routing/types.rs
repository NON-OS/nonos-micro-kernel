// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::sphinx::constants::{
    DELAY_LENGTH, HEADER_INTEGRITY_MAC_SIZE, NODE_ADDRESS_LENGTH, TRUNCATED_ROUTING_INFO_SIZE,
    VERSION_LENGTH,
};

/// Beta in the Sphinx paper: what one forwarding hop is told.
pub struct RoutingInformation {
    pub flag: u8,
    pub version: [u8; VERSION_LENGTH],
    pub node_address: [u8; NODE_ADDRESS_LENGTH],
    pub delay: [u8; DELAY_LENGTH],
    pub next_mac: [u8; HEADER_INTEGRITY_MAC_SIZE],
    pub next_routing: [u8; TRUNCATED_ROUTING_INFO_SIZE],
}
