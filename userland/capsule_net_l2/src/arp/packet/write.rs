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

use super::constants::{HLEN_MAC, HW_ETHERNET, PACKET_LEN, PLEN_IPV4, PROTO_IPV4};
use super::packet_type::ArpPacket;

impl ArpPacket {
    pub fn write(&self, out: &mut [u8]) -> bool {
        if out.len() < PACKET_LEN {
            return false;
        }
        out[0..2].copy_from_slice(&HW_ETHERNET.to_be_bytes());
        out[2..4].copy_from_slice(&PROTO_IPV4.to_be_bytes());
        out[4] = HLEN_MAC;
        out[5] = PLEN_IPV4;
        out[6..8].copy_from_slice(&self.oper.to_be_bytes());
        out[8..14].copy_from_slice(&self.sender_mac);
        out[14..18].copy_from_slice(&self.sender_ip);
        out[18..24].copy_from_slice(&self.target_mac);
        out[24..28].copy_from_slice(&self.target_ip);
        true
    }
}
