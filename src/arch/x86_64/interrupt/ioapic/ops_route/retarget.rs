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

use super::super::error::{IoApicError, IoApicResult};
use super::super::mmio::{redtbl_read, redtbl_write};
use super::super::ops_helpers::locate;

pub fn retarget(gsi: u32, dest_apic_id: u32) -> IoApicResult<()> {
    let (chip, idx) = locate(gsi).ok_or(IoApicError::GsiNotFound)?;
    // The RTE destination field is 8 bits wide. An APIC id above 255
    // (x2APIC) would be masked to 0xFF and retarget the wrong CPU; keep
    // the mask below but surface it so the mistarget is not silent.
    if dest_apic_id > 0xFF {
        crate::sys::serial::println(
            b"[IOAPIC] warning: retarget dest APIC id > 0xFF truncated to 8 bits (x2APIC)",
        );
    }
    unsafe {
        let (low, mut high) = redtbl_read(chip.mmio, idx);
        high &= !(0xFF << 24);
        high |= (dest_apic_id & 0xFF) << 24;
        redtbl_write(chip.mmio, idx, low, high);
    }
    Ok(())
}
