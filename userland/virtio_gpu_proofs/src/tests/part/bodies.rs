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

//! Response bodies, appended to a header.

use super::answers::{CAPSET, PANEL, PANEL_CM};
use crate::constants::VG_MAX_SCANOUTS;

const EDID_MAGIC: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

fn le32(out: &mut Vec<u8>, values: &[u32]) {
    for v in values {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

/// Sixteen modes; only the first is enabled, at `PANEL`.
pub fn display_info(mut out: Vec<u8>) -> Vec<u8> {
    le32(&mut out, &[0, 0, PANEL.0, PANEL.1, 1, 0]);
    out.resize(24 + 24 * VG_MAX_SCANOUTS, 0);
    out
}

/// One 128-byte EDID base block with the physical size at bytes 21 and 22.
pub fn edid(mut out: Vec<u8>, bad_magic: bool) -> Vec<u8> {
    le32(&mut out, &[128, 0]);
    let mut block = [0u8; 128];
    block[..8].copy_from_slice(&EDID_MAGIC);
    if bad_magic {
        block[0] = 0x01;
    }
    block[21] = PANEL_CM.0;
    block[22] = PANEL_CM.1;
    out.extend_from_slice(&block);
    out
}

pub fn capset_info(mut out: Vec<u8>) -> Vec<u8> {
    le32(&mut out, &[CAPSET.0, CAPSET.1, CAPSET.2, 0]);
    out
}
