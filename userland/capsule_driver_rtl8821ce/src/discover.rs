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

//! Find the RTL8821CE on the PCI bus via the broker device list, matching the
//! Realtek vendor and the 8821CE device id, and take its memory BAR. The
//! registers are not in BAR0 (BAR0 is the IO port BAR); rtw88 maps bar_id 2, so
//! the first MMIO BAR is discovered by scanning rather than assumed at index 0.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::{PCI_DEVICE_RTL8821CE, PCI_VENDOR_REALTEK};

const MAX_DEVICES: usize = 96;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    /// The BAR index the registers are mapped from. The 8821CE puts them in a
    /// memory BAR that is not BAR0, so the index is discovered, not assumed.
    pub bar_index: u32,
    pub bar_size: u64,
}

/// Locate the wifi chip, or `None` when the board carries a different adapter.
pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if r.bus_kind != BUS_KIND_PCI
            || r.vendor != PCI_VENDOR_REALTEK
            || r.device != PCI_DEVICE_RTL8821CE
        {
            continue;
        }
        // The registers are in a memory BAR, which on this chip is not BAR0.
        // Take the first MMIO BAR with a real size, whatever its index.
        let count = core::cmp::min(r.bar_count as usize, r.bars.len());
        for (i, bar) in r.bars[..count].iter().enumerate() {
            if bar.kind == BAR_KIND_MMIO && bar.size != 0 {
                return Some(Found {
                    device_id: r.device_id,
                    bar_index: i as u32,
                    bar_size: bar.size,
                });
            }
        }
    }
    None
}
