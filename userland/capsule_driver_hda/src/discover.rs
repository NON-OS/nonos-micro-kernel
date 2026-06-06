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

use crate::constants::{CLASS_AUDIO, HDA_BAR_INDEX, HDA_BAR_MIN_SIZE};

const MAX_DEVICES: usize = 32;
const PCI_CLASS_MULTIMEDIA: u8 = 0x04;
const PCI_SUBCLASS_HDA: u8 = 0x03;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub bar_size: u64,
}

pub fn find_hda() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(CLASS_AUDIO, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if is_candidate(r) {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                bar_size: r.bars[HDA_BAR_INDEX as usize].size,
            });
        }
    }
    None
}

fn is_candidate(r: &DeviceRecord) -> bool {
    let bar = r.bars[HDA_BAR_INDEX as usize];
    r.bus_kind == BUS_KIND_PCI
        && r.class == CLASS_AUDIO
        && r.pci_class == PCI_CLASS_MULTIMEDIA
        && r.pci_subclass == PCI_SUBCLASS_HDA
        && r.bar_count > HDA_BAR_INDEX
        && r.irq_pin != 0
        && r.irq_line != 0xff
        && bar.kind == BAR_KIND_MMIO
        && bar.size >= HDA_BAR_MIN_SIZE
}
