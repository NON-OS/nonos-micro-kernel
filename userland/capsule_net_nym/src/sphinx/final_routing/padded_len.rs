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
use crate::sphinx::filler::FILLER_STEP_SIZE_INCREASE;

/// How long the final block is before the filler is appended. It shrinks by
/// one slot per hop precisely so that adding the filler brings it back to the
/// full routing-info size, whatever the route length.
pub fn padded_len(route_len: usize) -> Option<usize> {
    if route_len == 0 {
        return None;
    }
    let consumed = FILLER_STEP_SIZE_INCREASE.checked_mul(route_len - 1)?;
    ENCRYPTED_ROUTING_INFO_SIZE.checked_sub(consumed)
}
