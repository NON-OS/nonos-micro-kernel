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

use crate::sphinx::constants::{ENCRYPTED_ROUTING_INFO_SIZE, TRUNCATED_ROUTING_INFO_SIZE};

/// Drop the tail a hop will never see. The next hop refills that space from
/// the filler, so the header stays one fixed length end to end.
pub fn truncate(enc: &[u8; ENCRYPTED_ROUTING_INFO_SIZE]) -> [u8; TRUNCATED_ROUTING_INFO_SIZE] {
    let mut out = [0u8; TRUNCATED_ROUTING_INFO_SIZE];
    out.copy_from_slice(&enc[..TRUNCATED_ROUTING_INFO_SIZE]);
    out
}
