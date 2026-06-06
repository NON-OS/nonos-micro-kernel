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

extern crate alloc;

use alloc::vec::Vec;

use super::init::init;
use super::types::{IsoFlags, MadtIoApic, MadtIso, MadtNmi};
use crate::arch::x86_64::acpi::parser::with_data;

pub fn init_from_acpi() -> Result<usize, ()> {
    let built = with_data(|data| {
        let ioapics: Vec<MadtIoApic> = data
            .ioapics
            .iter()
            .map(|io| MadtIoApic { phys_base: io.address, gsi_base: io.gsi_base })
            .collect();
        let isos: Vec<MadtIso> = data
            .overrides
            .iter()
            .map(|o| MadtIso {
                bus_irq: o.source_irq,
                gsi: o.gsi,
                flags: IsoFlags::from_polarity_trigger(o.polarity, o.trigger_mode),
            })
            .collect();
        let nmis: Vec<MadtNmi> = data
            .nmis
            .iter()
            .map(|n| MadtNmi {
                cpu: n.processor_uid,
                lint: n.lint,
                flags: IsoFlags::from_polarity_trigger(
                    (n.flags & 0x3) as u8,
                    ((n.flags >> 2) & 0x3) as u8,
                ),
            })
            .collect();
        (ioapics, isos, nmis)
    });
    let (ioapics, isos, nmis) = built.ok_or(())?;
    if ioapics.is_empty() {
        return Err(());
    }
    let count = ioapics.len();
    unsafe { init(&ioapics, &isos, &nmis) }.map_err(|_| ())?;
    Ok(count)
}
