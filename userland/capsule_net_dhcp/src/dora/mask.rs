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

// Convert a contiguous DHCP `subnet_mask` option into a CIDR
// prefix length. A discontiguous mask is rejected — that's a
// malformed server reply.
pub fn mask_to_prefix(mask: &[u8; 4]) -> Option<u8> {
    let bits = u32::from_be_bytes(*mask);
    if bits == 0 {
        return None;
    }
    let prefix = bits.leading_ones();
    let trailing = bits.trailing_zeros();
    if prefix + trailing == 32 {
        Some(prefix as u8)
    } else {
        None
    }
}
