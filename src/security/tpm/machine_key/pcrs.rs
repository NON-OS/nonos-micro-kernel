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

//! Which PCRs the key is bound to.

use super::consts::PCR_SELECT_BYTES;

/// Firmware code, the boot manager the firmware measured, the Secure Boot
/// policy, and the kernel hash the bootloader extends. Not PCR 1 or 3: a
/// changed boot order or a docked laptop would cost the owner their volume.
pub const BOUND_PCRS: [u8; 4] = [0, 4, 7, 9];

pub(super) fn bitmap(pcrs: &[u8]) -> [u8; PCR_SELECT_BYTES] {
    let mut sel = [0u8; PCR_SELECT_BYTES];
    for &p in pcrs {
        let i = (p / 8) as usize;
        if i < PCR_SELECT_BYTES {
            sel[i] |= 1 << (p % 8);
        }
    }
    sel
}
