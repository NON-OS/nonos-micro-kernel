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
use super::claim::claim;
use crate::constants::HCCPARAMS1;
use crate::regs::mmio_read32;

// Extended-capability ID for USB legacy support (xHCI 1.2 §7.1).
const XECP_ID_LEGACY: u32 = 1;
const XECP_WALK_LIMIT: u32 = 256;

/// Claim the controller from BIOS/SMM before it is reset. On real firmware the
/// controller is frequently still owned by SMM through USB legacy support;
/// resetting or driving it without the USBLEGSUP handshake races SMM and can
/// lose the boot keyboard or wedge the reset. Controllers without a legacy
/// capability (e.g. QEMU) advertise no xECP and this is a no-op.
pub fn legacy_handoff(mmio_base: u64) {
    // HCCPARAMS1[31:16] is the xECP offset, in 32-bit words from the cap base.
    let xecp = (mmio_read32(mmio_base + HCCPARAMS1) >> 16) & 0xFFFF;
    if xecp == 0 {
        return;
    }
    let mut cap = mmio_base + (xecp as u64) * 4;
    for _ in 0..XECP_WALK_LIMIT {
        let dw0 = mmio_read32(cap);
        if dw0 & 0xFF == XECP_ID_LEGACY {
            claim(cap);
            return;
        }
        let next = (dw0 >> 8) & 0xFF;
        if next == 0 {
            return;
        }
        cap += (next as u64) * 4;
    }
}
