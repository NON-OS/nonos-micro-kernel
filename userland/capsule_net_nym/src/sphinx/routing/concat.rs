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

use super::types::RoutingInformation;
use crate::sphinx::constants::ENCRYPTED_ROUTING_INFO_SIZE;
use alloc::vec::Vec;

/// Field order is the wire order and is not ours to choose: flag, version,
/// address, delay, then the next hop's MAC and its truncated routing info.
pub fn concatenate(info: &RoutingInformation) -> Vec<u8> {
    let mut out = Vec::with_capacity(ENCRYPTED_ROUTING_INFO_SIZE);
    out.push(info.flag);
    out.extend_from_slice(&info.version);
    out.extend_from_slice(&info.node_address);
    out.extend_from_slice(&info.delay);
    out.extend_from_slice(&info.next_mac);
    out.extend_from_slice(&info.next_routing);
    out
}
