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

use crate::sphinx::constants::ENCRYPTED_ROUTING_INFO_SIZE;
use alloc::vec::Vec;

/// Append the filler. The result is the full routing-info block: the filler
/// occupies exactly the slots the earlier hops will strip on the way.
pub fn combine_with_filler(
    encrypted: &[u8],
    filler: &[u8],
) -> Option<[u8; ENCRYPTED_ROUTING_INFO_SIZE]> {
    if encrypted.len().checked_add(filler.len())? != ENCRYPTED_ROUTING_INFO_SIZE {
        return None;
    }
    let mut joined = Vec::with_capacity(ENCRYPTED_ROUTING_INFO_SIZE);
    joined.extend_from_slice(encrypted);
    joined.extend_from_slice(filler);
    let mut out = [0u8; ENCRYPTED_ROUTING_INFO_SIZE];
    out.copy_from_slice(&joined);
    Some(out)
}
