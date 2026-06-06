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

use crate::ipv4::seal_at;

use super::types::{CHECKSUM_OFFSET, HDR_LEN};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    OutputTooSmall,
}

// Write an ICMP message into `out`: 8-byte header (type/code/cksum +
// 4-byte rest) followed by `payload`. The checksum is sealed over
// the entire message before return. Returns the total wire length.
pub fn build(
    icmp_type: u8,
    code: u8,
    rest: [u8; 4],
    payload: &[u8],
    out: &mut [u8],
) -> Result<usize, BuildError> {
    let total = HDR_LEN + payload.len();
    if out.len() < total {
        return Err(BuildError::OutputTooSmall);
    }
    out[0] = icmp_type;
    out[1] = code;
    out[2] = 0;
    out[3] = 0;
    out[4..8].copy_from_slice(&rest);
    out[HDR_LEN..total].copy_from_slice(payload);
    seal_at(&mut out[..total], CHECKSUM_OFFSET);
    Ok(total)
}
