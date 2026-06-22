// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::constants::FIXED_PROOF_BYTES;
use super::take32::take32;
use super::types::TransparentProof;

pub fn parse(buf: &[u8]) -> Result<TransparentProof<'_>, &'static str> {
    if buf.len() < FIXED_PROOF_BYTES {
        return Err("transparent proof too small");
    }
    let depth = buf[128];
    let sibling_len = depth as usize * 32;
    let dirs_len = (depth as usize + 7) / 8;
    let total = FIXED_PROOF_BYTES + sibling_len + dirs_len;
    if buf.len() != total {
        return Err("transparent proof size invalid");
    }
    Ok(TransparentProof {
        commitment: take32(buf, 0).ok_or("transparent C malformed")?,
        nonce_point: take32(buf, 32).ok_or("transparent A malformed")?,
        z_x: take32(buf, 64).ok_or("transparent zx malformed")?,
        z_r: take32(buf, 96).ok_or("transparent zr malformed")?,
        depth,
        siblings: &buf[129..129 + sibling_len],
        dirs: &buf[129 + sibling_len..total],
    })
}
