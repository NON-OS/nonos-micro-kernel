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

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::{
    CLASS_DISPLAY, DEVICE_BGA, FB_BAR, PCI_CLASS_DISPLAY, REG_BAR, VENDOR_QEMU_BOCHS,
};

const MAX_DEVICES: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub fb_size: u64,
    pub reg_size: u64,
}

pub fn find_bga() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(CLASS_DISPLAY, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        let fb = r.bars[FB_BAR as usize];
        let reg = r.bars[REG_BAR as usize];
        if r.bus_kind == BUS_KIND_PCI
            && r.pci_class == PCI_CLASS_DISPLAY
            && r.vendor == VENDOR_QEMU_BOCHS
            && r.device == DEVICE_BGA
            && r.bar_count > REG_BAR
            && fb.kind == BAR_KIND_MMIO
            && reg.kind == BAR_KIND_MMIO
            && fb.size > 0
            && reg.size > 0
        {
            return Some(Found { device_id: r.device_id, fb_size: fb.size, reg_size: reg.size });
        }
    }
    None
}
