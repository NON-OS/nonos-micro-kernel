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

use super::padded_len::padded_len;
use super::types::FinalRoutingInformation;
use crate::crypto::random::fill_random;
use crate::sphinx::constants::FINAL_NODE_META_INFO_LENGTH;
use alloc::vec;
use alloc::vec::Vec;

/// The padding is random, not zeros: it occupies the space a longer route
/// would have used, and a mix must not be able to tell the two apart.
pub fn add_padding(info: &FinalRoutingInformation, route_len: usize) -> Option<Vec<u8>> {
    let total = padded_len(route_len)?;
    let pad = total.checked_sub(FINAL_NODE_META_INFO_LENGTH)?;
    let mut out = Vec::with_capacity(total);
    out.push(info.flag);
    out.extend_from_slice(&info.version);
    out.extend_from_slice(&info.destination);
    out.extend_from_slice(&info.identifier);
    let mut bytes = vec![0u8; pad];
    fill_random(&mut bytes).ok()?;
    out.extend_from_slice(&bytes);
    Some(out)
}
